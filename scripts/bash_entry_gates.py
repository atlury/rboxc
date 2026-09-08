"""Lower Bash main's nonlocal recovery into explicit Rust-compatible branches."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,re,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint

def outline(root,source,record):
 from split_entry import clean_args,extent,patch
 stage=root/'build/translation/bash';stage.mkdir(parents=True,exist_ok=True)
 flags=clean_args(record)
 adaptations=['-Wno-error','-std=gnu17','-include','stdbool.h','-Dunreachable()=__builtin_unreachable()',
              '-Dstatic_assert=_Static_assert','-Dalignof=_Alignof','-Dnullptr=((void*)0)']
 pp=stage/'shell.i'
 with pp.open('w') as out,(stage/'preprocess.log').open('w') as err:
  subprocess.run(['clang-21',*flags,*adaptations,'-E','-P'],cwd=record['directory'],stdout=out,stderr=err,check=True)
 def ast(args,path):
  with path.open('w') as out,path.with_suffix('.log').open('w') as err:
   subprocess.run(['clang-21',*args,'-fsyntax-only','-Xclang','-ast-dump=json'],cwd=record['directory'],stdout=out,stderr=err,check=True)
  return json.loads(path.read_text())
 original_ast=ast([*flags,*adaptations],stage/'original-main-ast.json')
 pp_ast=ast(['-x','c','-std=gnu17',str(pp)],stage/'preprocessed-main-ast.json')
 def mainbody(tree):
  main=next(n for n in tree['inner'] if n.get('name')=='main' and any(i.get('kind')=='CompoundStmt' for i in n.get('inner',[])))
  return next(i for i in main['inner'] if i.get('kind')=='CompoundStmt')
 body=mainbody(pp_ast);text=pp.read_text();a,b=extent(body)
 calls=[];parents={}
 def walk(n,chain):
  if n.get('kind')=='CallExpr':calls.append(n);parents[n['id']]=chain
  for child in n.get('inner',[]):walk(child,[*chain,n])
 walk(body,[])
 def callee(n):
  v=n['inner'][0]
  while v.get('kind')!='DeclRefExpr':
   assert len(v.get('inner',[]))==1,v.get('kind');v=v['inner'][0]
  return v['referencedDecl']['name']
 checkpoints=[n for n in calls if callee(n)=='__sigsetjmp'];assert len(checkpoints)==4
 labels={};statement_labels={}
 for index,n in enumerate(checkpoints,1):
  args=n['inner'][1:];target=text[slice(*extent(args[0]))].strip(' ()')
  assert target in ('top_level','subshell_top_level'),target
  save=int(text[slice(*extent(args[1]))]);assert save in (0,1)
  chain=parents[n['id']]
  compound=max(i for i,v in enumerate(chain) if v.get('kind')=='CompoundStmt')
  statement=chain[compound+1] if compound+1<len(chain) else n
  statement_labels[statement['id']]=index
  labels[n['id']]=(index,target,save)
 jumps='switch (r.target) {'+''.join(f'case {i}: goto rboxc_checkpoint_{i};' for i in range(1,5))+'default: abort();}'
 declarations=[];definitions=[];records=[]
 def declare(ty,name):
  assert not re.search(r'\[[^]]*\]',ty),ty
  if re.search(r'\(\s*\*\s*\)',ty):return re.sub(r'\(\s*\*\s*\)','(*'+name+')',ty,count=1)
  return ty+' '+name
 temporary_count=0
 def temporary():
  nonlocal temporary_count
  temporary_count+=1
  return 'rboxc_expr_'+str(temporary_count)
 def raw(n):return text[slice(*extent(n))]
 def render_call(n):
  if n['id'] in labels:
   index,target,save=labels[n['id']]
   return '',f'rboxc_gate_checkpoint (&rboxc_gate, {index}, {int(target=="subshell_top_level")}, {save})'
  args=n['inner'][1:];pre='';values=[]
  for argument in args:
   before,value=expression(argument);pre+=before;values.append(value)
  index=len(records);records.append(None)
  name=callee(n);ty=n['type']['qualType']
  argtypes=[v['type']['qualType'] for v in args]
  result=f'rboxc_gate_result_{index}';fn=f'rboxc_gate_call_{index}'
  structure='struct '+result+' { int jumped; int target; int code;'+(' '+declare(ty,'value')+';' if ty!='void' else '')+' };'
  signature='struct '+result+' '+fn+' (struct rboxc_bash_gate *g'+''.join(', '+declare(t,'a'+str(i)) for i,t in enumerate(argtypes))+')'
  declarations.extend([structure,signature+';'])
  call=name+' ('+', '.join('a'+str(i) for i in range(len(args)))+')'
  definitions.append(signature+''' {
    int saved_errno = errno;
    int code = sigsetjmp (top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 0);
      struct '''+result+''' result = {0};
      result.jumped = 1; result.target = g->top; result.code = code;
      return result;
    }
    code = sigsetjmp (subshell_top_level, 0);
    if (code) {
      rboxc_gate_caught (g, 1);
      struct '''+result+''' result = {0};
      result.jumped = 1; result.target = g->sub; result.code = code;
      return result;
    }
    sigprocmask (SIG_SETMASK, &g->allowed, 0);
    errno = saved_errno;
    struct '''+result+''' result = {0};
    '''+('result.value = ' if ty!='void' else '')+call+''';
    rboxc_gate_block (g);
    return result;
  }
''')
  records[index]={'callee':name,'result':ty,'arguments':argtypes,'source_expression_sha256':__import__('hashlib').sha256(text[slice(*extent(n))].encode()).hexdigest()}
  variable='rboxc_result_'+str(index)
  check='if ('+variable+'.jumped) { rboxc_gate.pending = '+variable+'.code; rboxc_gate.target = '+variable+'.target; '+jumps.replace('r.target',variable+'.target')+' }\n'
  pre+='struct '+result+' '+variable+' = '+fn+' (&rboxc_gate'+''.join(', ('+v+')' for v in values)+');\n'+check
  return pre,variable+'.value' if ty!='void' else '(void)0'
 def expression(n):
  kind=n.get('kind');children=[c for c in n.get('inner',[]) if 'range' in c]
  if kind=='CallExpr':return render_call(n)
  if kind=='GenericSelectionExpr':
   selected=next(c for c in n['inner'] if c.get('selected'))
   return expression(selected['inner'][-1])
  if kind=='UnaryExprOrTypeTraitExpr':return '',raw(n)
  if kind=='BinaryOperator' and n.get('opcode') in ('&&','||'):
   left,right=children;before,lhs=expression(left);after,rhs=expression(right);tmp=temporary()
   code=before+'int '+tmp+' = !!('+lhs+');\n'
   code+='if ('+('!' if n['opcode']=='||' else '')+tmp+') {\n'+after+tmp+' = !!('+rhs+');\n}\n'
   return code,tmp
  if kind=='ConditionalOperator':
   cond,yes,no=children;before,value=expression(cond);p1,v1=expression(yes);p2,v2=expression(no);tmp=temporary()
   code=before+declare(n['type']['qualType'],tmp)+';\nif ('+value+') {\n'+p1+tmp+' = ('+v1+');\n} else {\n'+p2+tmp+' = ('+v2+');\n}\n'
   return code,tmp
  pre='';edits=[];start,end=extent(n)
  for c in children:
   x,y=extent(c);before,value=expression(c);pre+=before
   edits.append((x-start,y-start,'('+value+')'))
  return pre,patch(text[start:end],edits)
 def statement(n):
  kind=n.get('kind');children=n.get('inner',[])
  prefix=('rboxc_checkpoint_'+str(statement_labels[n['id']])+': ;\n') if n.get('id') in statement_labels else ''
  if kind=='CompoundStmt':result='{\n'+''.join(statement(c) for c in children)+'}\n'
  elif kind=='IfStmt':
   assert len(children) in (2,3)
   before,value=expression(children[0]);result=before+'if ('+value+') {\n'+statement(children[1])+'}\n'
   if len(children)==3:result+='else {\n'+statement(children[2])+'}\n'
  elif kind=='WhileStmt':
   before,value=expression(children[0]);result='while (1) {\n'+before+'if (!('+value+')) break;\n'+statement(children[1])+'}\n'
  elif kind=='ForStmt':
   init,empty,cond,incr,loop=children;assert not empty
   parts=[expression(c) for c in (init,cond,incr)];assert not any(p for p,v in parts)
   result='for ('+'; '.join(v for p,v in parts)+') {\n'+statement(loop)+'}\n'
  elif kind=='DeclStmt':
   def has_call(v):return v.get('kind')=='CallExpr' or any(has_call(c) for c in v.get('inner',[]))
   assert not has_call(n), 'review initializer evaluation'
   result=raw(n)+'\n'
  elif kind=='NullStmt':result=';\n'
  elif kind in ('BreakStmt','ContinueStmt','GotoStmt'):result=raw(n)+';\n'
  elif kind=='ReturnStmt':
   before,value=expression(children[0]);result=before+'return '+value+';\n'
  else:
   assert kind and ('type' in n),kind
   before,value=expression(n);result=before+value+';\n'
  return prefix+result
 translated=statement(body)
 assert '__sigsetjmp' not in translated
 translated=translated[:1]+'\nstruct rboxc_bash_gate rboxc_gate = {0};\nrboxc_gate_begin (&rboxc_gate);\n'+translated[1:]
 gate='''
/* Every GNU call runs under live C jump checkpoints. Rust computation keeps
   asynchronous signals blocked; the logical GNU mask is restored for calls. */
struct rboxc_bash_gate {
  sigset_t allowed, top_mask, sub_mask;
  int top, sub, top_save, sub_save, pending, target;
};
void rboxc_gate_begin (struct rboxc_bash_gate *g) {
  int saved = errno; sigset_t all; sigfillset (&all);
  sigprocmask (SIG_BLOCK, &all, &g->allowed); errno = saved;
}
void rboxc_gate_block (struct rboxc_bash_gate *g) {
  int saved = errno; sigset_t all; sigfillset (&all);
  sigprocmask (SIG_BLOCK, &all, &g->allowed); errno = saved;
}
void rboxc_gate_caught (struct rboxc_bash_gate *g, int sub) {
  int saved = errno;
  if (sub ? g->sub_save : g->top_save)
    sigprocmask (SIG_SETMASK, sub ? &g->sub_mask : &g->top_mask, 0);
  rboxc_gate_block (g); errno = saved;
}
int rboxc_gate_checkpoint (struct rboxc_bash_gate *g, int id, int sub, int save) {
  if (g->target == id) { int code = g->pending; g->target = g->pending = 0; return code; }
  if (sub) { g->sub = id; g->sub_save = save; g->sub_mask = g->allowed; }
  else { g->top = id; g->top_save = save; g->top_mask = g->allowed; }
  return 0;
}
'''
 original=source.read_text();x,y=extent(mainbody(original_ast))
 original=original[:x]+translated+original[y:]
 anchor='#if defined (NO_MAIN_ENV_ARG)';assert original.count(anchor)==2
 position=original.rindex(anchor)
 original=original[:position]+gate+'\n'+'\n'.join(declarations)+'\n'+original[position:]
 original+='\n'+'\n'.join(definitions)
 report={'scope':'Pinned Bash main is preprocessed and structurally lowered. Each original call executes inside C jump checkpoints and returns a typed outcome. Recovery branches become explicit goto edges for C2Rust; no nonlocal jump traverses Rust. Asynchronous signals are deferred across Rust computation and the GNU logical signal mask is restored at each call boundary.',
  'source_sha256':fingerprint(source),'preprocessed_sha256':fingerprint(pp),'main_source_bytes':b-a,
  'calls':records,'checkpoints':[{'id':i,'target':t,'save_mask':s} for i,t,s in labels.values()],
  'driver_sha256':fingerprint(Path(__file__))}
 (root/'evidence/bash-call-gates.json').write_text(json.dumps(report,indent=2)+'\n')
 return original,[{'call_gate_report':'evidence/bash-call-gates.json','sha256':fingerprint(root/'evidence/bash-call-gates.json')}]
