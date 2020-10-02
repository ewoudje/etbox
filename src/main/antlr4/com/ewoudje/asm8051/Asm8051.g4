grammar Asm8051;

file: (directive | instruction | label | data | PREPROCESSOR | NL)+ ('end' NL*)?  EOF;

instruction: instr (parameter (',' parameter (',' parameter)?)?)?;
label: IDENTIFIER ':';
data: ('db' | 'DB') expressionlist;
directive: org | equ | bit_d;

expressionlist: expression (',' expression)*;

expression: string | value;

equ: IDENTIFIER EQU const_values;
bit_d: IDENTIFIER BIT (bit | const_values);
org: ORG calc_constant;

parameter: shortRegister
         | register
         | constant
         | value
         | bit
         ;

bit: (register | shortRegister) BITSELECT;
shortRegister: SRN | sfr | PTRR;
register: value;
constant: HASH calc_constant;
locAt: '@' ((PTRR | SFR_DPTR) | (SFR_ACC '+' SFR_DPTR));

calc_constant: const_values                     #const
             | const_values '-' calc_constant   #const_sub
             | const_values '+' calc_constant   #const_add
             ;


value: const_values | locAt;
const_values: decimal | hexadecimal | binary | char_ | instr_loc | labelP;

labelP: IDENTIFIER;

string: STRING;
decimal:  DECIMAL;
hexadecimal: HEXADECIMAL;
binary: BINARY;
char_: CHAR;
instr_loc: INSTR_LOC;

sfr: SFR_ACC | SFR_DPTR;

instr: ('ADD'   | 'add'   ) #add
     | ('ADDC'  | 'addc'  ) #addc
     | ('SUBB'  | 'subb'  ) #subb
     | ('INC'   | 'inc'   ) #inc
     | ('DEC'   | 'dec'   ) #dec
     | ('MUL'   | 'mul'   ) #mul
     | ('DIV'   | 'div'   ) #div
     | ('DA'    | 'da'    ) #da
     | ('ANL'   | 'anl'   ) #anl
     | ('ORL'   | 'orl'   ) #orl
     | ('XRL'   | 'xrl'   ) #xrl
     | ('CLR'   | 'clr'   ) #clr
     | ('CPL'   | 'cpl'   ) #cpl
     | ('RL'    | 'rl'    ) #rl
     | ('RLC'   | 'rlc'   ) #rlc
     | ('RR'    | 'rr'    ) #rr
     | ('RRC'   | 'rrc'   ) #rrc
     | ('SWAP'  | 'swap'  ) #swap
     | ('MOV'   | 'mov'   ) #mov
     | ('MOVC'  | 'movc'  ) #movc
     | ('MOVX'  | 'movx'  ) #movx
     | ('PUSH'  | 'push'  ) #push
     | ('POP'   | 'pop'   ) #pop
     | ('XCH'   | 'xch'   ) #xch
     | ('XCHD'  | 'xchd'  ) #xchd
     | ('SETB'  | 'setb'  ) #setb
     | ('ACALL' | 'acall' ) #acall
     | ('LCALL' | 'lcall' ) #lcall
     | ('RET'   | 'ret'   ) #ret
     | ('RETI'  | 'reti'  ) #reti
     | ('AJMP'  | 'ajmp'  ) #ajmp
     | ('LJMP'  | 'ljmp'  ) #ljmp
     | ('SJMP'  | 'sjmp'  ) #sjmp
     | ('JZ'    | 'jz'    ) #jz
     | ('JNZ'   | 'jnz'   ) #jnz
     | ('JC'    | 'jc'    ) #jc
     | ('JNC'   | 'jnc'   ) #jnc
     | ('JB'    | 'jb'    ) #jb
     | ('JNB'   | 'jnb'   ) #jnb
     | ('JBC'   | 'jbc'   ) #jbc
     | ('CJNE'  | 'cjne'  ) #cjne
     | ('DJNZ'  | 'djnz'  ) #djnz
     | ('NOP'   | 'nop'   ) #nop
     ;

INCLUDE_PREPROCESSOR: HASH 'include' WS STRING NL;

HASH: '#';

INSTR_LOC: '$';

EQU: ('equ' | 'EQU') | ('data' | 'DATA');
ORG: ('org' | 'ORG');
BIT: ('bit' | 'BIT');

CHAR: '\u0027' ~'\u0027'* '\u0027';

STRING: '"' ~'"'* '"';

COMMENT: ';' ~[\r\n]* '\r'? '\n' -> skip;

SRN: 'R''2'..'7';
PTRR: 'R''0'..'1';

SFR_ACC: 'A' | 'a';
SFR_DPTR: 'DPTR' | 'dptr';

BITSELECT: '.''0'..'7';

DECIMAL: ('0'..'9')+;

IDENTIFIER: [a-zA-Z_] [a-zA-Z0-9_]*;

HEXADECIMAL: ('0'..'f' | '0'..'F')+ ('h' | 'H')
           | '0x' ('0'..'f' | '0'..'F')+
           ;
BINARY: '0'..'1'+ ('b' | 'B');

WS: (' ' | '\t')+ -> skip;
NL:  '\r'? '\n';
