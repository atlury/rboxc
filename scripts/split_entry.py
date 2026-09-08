"""Keep GNU nonlocal-jump helper bodies in C while translating command entry logic."""
# SPDX-License-Identifier: GPL-3.0-or-later
import copy,json,re,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint

COMMANDS={'tftpd','tftp','ftpd','telnet','bash'}
def clean_args(record):
 result=[];skip=False
 for word in record['arguments'][1:]:
  if skip:skip=False;continue
  if word in ('-o','-MF','-MT'):skip=True;continue
  if word in ('-c','-MD','-MP','-MMD'):continue
  result.append(word)
 return result

def location(node):return node.get('expansionLoc',node)
def extent(node):
 a=location(node['range']['begin']);b=location(node['range']['end'])
 return a['offset'],b['offset']+b['tokLen']
def patch(text,edits):
 # Multiple declarations may share a storage-class token; apply it only once.
 edits=sorted(set(edits),reverse=True);limit=len(text)
 for a,b,replacement in edits:
  assert 0<=a<=b<=limit,(a,b,limit)
  text=text[:a]+replacement+text[b:];limit=a
 return text

def outline(name,text):
 """Extract only explicitly reviewed jump regions from three command entries."""
 helpers=[]
 if name=='tftp':
  start=text.index('  signal (SIGINT, intr);',text.index('main ('))
  end=text.index('\n}\n',start)
  body=text[start:end]
  assert body.count('setjmp (toplevel)')==2
  helpers.append(('void rboxc_native_tftp_loop (void)',body))
  text=text[:start]+'  rboxc_native_tftp_loop ();'+text[end:]
 if name=='ftpd':
  start=text.index('  setjmp (errcatch);',text.index('main ('))
  end=text.index('\n}\n',start);body=text[start:end]
  assert body.count('yyparse ();')==1
  helpers.append(('void rboxc_native_ftpd_loop (void)',body))
  text=text[:start]+'  rboxc_native_ftpd_loop ();'+text[end:]
 if name=='telnet':
  start=text.index('      if (setjmp (toplevel) != 0)',text.index('main ('))
  end=text.index('      /* NOT REACHED */',start)
  body=text[start:end].replace('tn (argp - args, args)','tn (count, args)')
  helpers.append(('int rboxc_native_telnet_connect (int count, char **args)',body))
  text=text[:start]+'      return rboxc_native_telnet_connect (argp - args, args);\n'+text[end:]
  start=text.index('  setjmp (toplevel);',text.index('main ('))
  end=text.index('\n}\n',start);body=text[start:end]
  helpers.append(('void rboxc_native_telnet_loop (void)',body))
  text=text[:start]+'  rboxc_native_telnet_loop ();'+text[end:]
 if helpers:
  # Prototypes precede main; helper definitions follow the original source.
  position=text.index('\nmain (');position=text.rfind('\n',0,position)+1
  text=text[:position]+''.join(signature+';\n' for signature,_ in helpers)+text[position:]
  text+='\n'+''.join(signature+'\n{\n'+body+'\n}\n' for signature,body in helpers)
 return text,[{'signature':sig,'body_sha256':__import__('hashlib').sha256(body.encode()).hexdigest()} for sig,body in helpers]

def prepare(root,name,source,record):
 stage=root/'build/translation'/name;stage.mkdir(parents=True,exist_ok=True)
 original=source.read_text()
 if name=='bash':
  from bash_entry_gates import outline as bash_outline
  text,regions=bash_outline(root,source,record)
 else:text,regions=outline(name,original)
 outlined=stage/'outlined.c';outlined.write_text(text)
 flags=clean_args(record)
 # The recorded source may be relative to its original compiler directory.
 flags=[str(outlined) if (Path(record['directory'])/w).resolve()==source else w for w in flags]
 assert flags.count(str(outlined))==1
 flags+=['-iquote',str(source.parent)]
 adaptations=['-Wno-error','-std=gnu17','-include','stdbool.h','-Dunreachable()=__builtin_unreachable()',
              '-Dstatic_assert=_Static_assert','-Dalignof=_Alignof','-Dnullptr=((void*)0)']
 ast=stage/'split-ast.json'
 with ast.open('w') as out,(stage/'split-ast.log').open('w') as err:
  subprocess.run(['clang-21',*flags,*adaptations,'-Xclang','-ast-dump=json','-fsyntax-only'],cwd=record['directory'],stdout=out,stderr=err,check=True)
 nodes=[n for n in json.loads(ast.read_text())['inner'] if n.get('kind') in ('FunctionDecl','VarDecl')
        and 'offset' in n.get('loc',{}) and 'includedFrom' not in n['loc']]
 native_edits=[];rust_edits=[];functions=[];main=None;storage=set()
 for n in nodes:
  a,b=extent(n);loc=n['loc']['offset'];symbol=n['name']
  assert text[loc:loc+len(symbol)]==symbol,(name,symbol)
  if n.get('storageClass')=='static':
   match=re.search(r'\bstatic\b',text[a:loc]);assert match
   x,y=a+match.start(),a+match.end();native_edits.append((x,y,''));storage.add((x,y))
  if n['kind']=='FunctionDecl':
   body=next((v for v in n.get('inner',[]) if v['kind']=='CompoundStmt'),None)
   if not body:continue
   x,y=extent(body)
   if symbol=='main':
    assert main is None;main=(x,y);native_edits.append((x,y,';'))
   else:rust_edits.append((x,y,';'));functions.append(symbol)
  else:
   # Keep structure definitions and declarator spelling; remove only initializers.
   if n.get('init'):
    eq=text.index('=',loc,b)
    # Conditional initializers may end before their closing #endif. Retain
    # directive nesting when removing the selected definition's initializer.
    directives=re.findall(r'^\s*#[^\n]*$',text[eq:b],re.M)
    rust_edits.append((eq,b,'\n'+'\n'.join(directives)+'\n' if directives else ''))
   if n.get('storageClass')!='extern':
    if n.get('storageClass')=='static':
     x,y=next((x,y) for x,y in storage if a<=x<loc);rust_edits.append((x,y,'extern'))
    else:rust_edits.append((a,a,'extern '))
   # An unsized initialized array becomes a complete external array declaration.
   match=re.match(r'\s*\[\s*\]',text[loc+len(symbol):b])
   if match:
    count=re.search(r'\[(\d+)\]$',n['type']['qualType']);assert count
    x=loc+len(symbol);rust_edits.append((x,x+match.end(),'['+count[1]+']'))
 # Source-local static function prototypes and definitions must refer to C helpers.
 for x,y in storage:
  if not any(a==x and b==y for a,b,_ in rust_edits):rust_edits.append((x,y,'extern'))
 assert main and functions
 native=stage/'native-helpers.c';native.write_text(patch(text,native_edits))
 entry_text=patch(text,rust_edits)
 if name=='ftpd':
  assert entry_text.count('#define FTP_NAMES')==1
  entry_text=re.sub(r'^#define FTP_NAMES[^\n]*$', '#undef FTP_NAMES /* GNU tables are owned by native-helpers.c. */',entry_text,flags=re.M)
 entry=stage/'entry.c';entry.write_text(entry_text)
 obj=stage/'native-helpers.o'
 args=['gcc',*[str(native) if w==str(outlined) else w for w in flags],'-c','-o',str(obj)]
 with (stage/'native-helpers-build.log').open('w') as log:
  subprocess.run(args,cwd=record['directory'],stdout=log,stderr=subprocess.STDOUT,check=True)
 symbols=subprocess.check_output(['nm','-g','--defined-only',obj],text=True)
 assert not re.search(r'\bmain$',symbols,re.M)
 result={'scope':'The command main body is translated; source-local non-main functions and globals remain isolated GNU C helpers. Explicit nonlocal-jump regions are outlined with all checkpoint/transfer calls inside C. Original C main has no definition in the native helper object.',
  'source':str(source),'source_sha256':fingerprint(source),'entry_source':str(entry),'entry_sha256':fingerprint(entry),
  'native_source':str(native),'native_source_sha256':fingerprint(native),'native_object':str(obj),'native_object_sha256':fingerprint(obj),
  'native_functions':functions,'outlined_jump_regions':regions,'main_source_bytes':main[1]-main[0],
  'gcc_arguments':args,'directory':record['directory'],'driver_sha256':fingerprint(Path(__file__))}
 (root/f'evidence/{name}-split-entry.json').write_text(json.dumps(result,indent=2)+'\n')
 translated=copy.deepcopy(record);translated['file']=str(entry)
 translated['arguments']=['clang-21',*[str(entry) if w==str(outlined) else w for w in flags],'-c','-o',record['output']]
 return translated,result
