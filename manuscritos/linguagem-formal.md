___
# Linguagem Formal

Entende-se por linguagem formal o estudo de modelos matemáticos que possibilitam a especificação e o reconhecimento de linguagens (no sentido amplo da palavra), suas classificaçoes, estruturas, propriedades, caracteristicas e inter-relacionamentos.

A importancia dessa teoria na ciencia da computação é dupla: ela tanto ela tanto apoia outros aspectos teóricos da ciência da computação (decidibilidade,computabilidade,complexidade computacional, por exemplo), como fundamenta diversas aplicaçoes computacionais, tais como processamento de linguagens, reconhecimento de padrões, modelagem de sistemas.

Para definir o que a teoria das linguagens formais é preciso definir o que é linguagem e  o que é linguagem formal. Inicialmente, de maneira bastante informal podemos definir linguagem como sendo uma forma de comunicação. Elaborando um pouco mais esta definição, podemos definir uma linguagem como sendo um "cojunto de elementos(simbolos) e um conjunto de metodos(regras) para combinar estes elementos, usado e entendido por uma determinada comunidade". São exemplos de as "linguagens naturais"(ou idiomas), "linguagens de programação" e os "protocolos de comunicação".

Assim podemos dizer que "linguagens formais" são mecanismos formais para representação de especificação de linguagens baseadas na chamada "teoria da computação". As representações podem ser feitas por reconhecedores e geradores. Os reconhecedores são dispositvos formais que servem para verificar se uma frase pertence ou não a uma determinada linguagem. São os autômatos: autômatos finitos, autômatos de pilha  e máquina de Turing. Os sistemas geradores são dispositivos são dispositvos formais que permitem a geração sistematica de todas as frases de uma linguagem. Os principais sistemas geradores disponiveis sao as gramáticas, onde se destacam as gramáticas de Chonsky. Então, linguagens formais podem ser representadas de maneira finita e precisa através de sistemas com sustentação matemática.

## História da linguagem nao formal

Acredita-se que a primeira linguagem formal seja utilizada por Gottlob Frege em seu Begriffsschrift(1879), que literalmente significa "escrita conceito", e que Frege descreveu como uma "linguagem formal do pensamento puro". 

O sistema Semi-Thue de Axel Thue, que pode ser usado para reescrever cadeias, foi influente em gramáticas formais.

## Palavras sobre um alfabeto

Um alfabeto, no contexto de linguagens formais, pode ser qualquer conjunto, embora muitas vezes, faça sentido usar alfabeto no sentido usual da palavra, ou, mais geralmente, um conjunto de caracteres, como ASCII ou Unicode. Alfabetos também podem ser infinitos, por exemplo, a lógica de primeira ordem é frequentemente expressa utilizando um alfabeto que, além de símbolos tais  como  ^, ¬  Æ  e , entre parenteses, contém muitos elementos que se estendem ao infinito,  x0,x1,x2,x3..., que desempenham o papel de variáveis. Os elementos de um alfabeto são chamados de letras.Uma palavra sobre um alfabeto pode ser qualquer sequencia, ou cadeia finita de caracteres, que por vezes podem incluir espaços, e são separados por caracteres de separação de palavras especificadas. O conjunto de todas as palavras sobre um alfabeto ∑ é geralmente indicado por ∑*(usando o fechamento de Kleene). o comprimento de uma palavra é o número  de caracteres ou letras de que é composto. Para qualquer alfabeto só há uma palavra de comprimento 0, a palavra vazia, o que é muitas vezes denotado por e , ε ou ꟛ. Por concatenação pode-se combinar duas palavras e forma uma nova palavra,  cujo comprimento é igual á soma dos comprimentos das palavras originais. O resultado de uma concatenação de uma palavra com uma palavra vazia é a palavra original.

Em algumas aplicações, espacialmente logica, o alfabeto é  também conhecido como vocabulário e as palavras são conhecidas como formulas ou sentenças, isso quebra a letra/palavra metáfora e a substitui por uma palavra/sentença metáfora 

## Definição

Uma linguagem formal L sobre um alfabeto ∑ é um subconjunto de ∑*, isto é, um conjunto de palavras sobre um alfabeto. Por vezes, os conjuntos de palavras são agrupados em expressões, que as regras e as cosntantes podem ser formuladas para a criação de "expressõe bem formadas". Em ciência e matemática de computação, que não costuman lidar com lingaugens naturais, o objetivo "formal" é freqentemente omitido como redundante.

Enquanto na teoria da lingaugem formal, geralmente se preucupa com lingaugens formais que são descritas por algumas regras sintáticas, a própia definição do conceito de "linguagem formal" é apenas é apenas como mencionado acima: um (possivelmente infinito) conjunto de cadeias de tamanho finito, compsoto de um determinado alfabeto, nem mais nem menos. Na prítica, há muitas linguas que podem ser descritas pelas regras, tais como linguagens regulares e linguagens livres de contexto. A noção de uma gramatica formal pode estar mais perto do conceito intuitivo de uma "linguagem", que é descrita por regras sintáticas. Por um abuso de definição, uma particular linguagem formal, é muitas vezes consederada como sendo equipada com uma gramática formal que a descreve.

## Exemplos

As seguintes regras descrevem uma linguaggem L formal sobre o alfabeto ∑ = {0,1,2,3,4,5,6,7,8,9,+,=}:

- Cada cadeia não vazia que não contem "+" ou "="  e não começa com "0" está em L.

- Uma sequência contendo "=" está em L se e somente se há exatamente, um "=", e ela separa duas cadeias validas de L.

- Uma sequência contendo "+", mas não "=" está em L se e somente se todos os "+" na cadeia separam duas cadeias válidas de L.

- Nenhuma cadeia está em L que não as sugeridas pelas regras anteriores.

Sob estas regras, a cadeia "23 + 4 = 555" esta em L, mas a cadeia "= 234 = +" não está. Esta linguagem formal expressa números naturais, declaraçoes de adição bem formadas, e igualdades adições bem formadas, mas estas exprimen apenas o que elas se parecem (sua sintaxe), não o que eles querem dizer(semântica) . Por exemplo, em nenhuma parte destas regras existe qualquer indicação que "0" signfica zero ou que "+" signifca adição.

## Construções

Para linguagens finitas, uma pode explicitamente enumerar todas as palavras bem formdas. Por exemplo, pode-se descrever uma linguagem L somente como L = { "a", "b","c","ABC"}. O degenerado caso desta construção é a linguagem vazia, que não contém nunhuma palavra (L= ø ). No entanto, mesmo ao longo de uma alfabeto (não-vazio) finito, como ∑ = {"a","b"} existem infinitamente muitas palavras: "a","abb","aaaaabababaaaaa",...  Portanto, as linguagens formais são tipicamente infinitas, e descrever uma linguagem formal infinita não é tão simples como escrever L={"a","b","c"}. Aqui estão alguns exemplos de linguagens formais:

- L =  ∑*, o consjunto de todas as plavras sobre ∑;

- L = {"a"}* ={"a"n}, onde _n_ varia ao longo dos números  naturais e "a" significa "a" repetido  _n_ vezes (isto é, o conjunto de palavras que consiste apenas pelo simbolo "a");

- o conjunto de programas sintaticamente corretos em uma determinada linguagem de programação (a sintaxe do que é normalmente definido por uma gramática livre contexto);

- o conjunto de entradas nos quais uma determinada maquina de turing para, ou

- o conjunto fde sequências máximas de caracteres alfanúmericos ASCII nesta linha , i.e, conjunto {"o","conjunto","de","sequências","máximas","cordas","alfanuméricas","ASCII","nesta","linha","i","e"}.

## Formalismos de especificações de linngaugens

A teoria da lingueagem formal, raramente se preucupa com determindas línguas(exceto como exemplos), mas esta preucupada principalmente com o estudos de vários tipos de formalismos para descrever línguas. Por exemplo, uma linguagem pode ser dada como

- aquelas cadeias de caracteres gerados por alguma gramática formal;

- aquelas cadeias descritas ou acompanhadas por uma determinada expresão regular;

- aquelas cadeias descritas por alguns autômatos, como uma máquina de Turing ou autômato de estados finitos;

- aquelas cadeias para qual algum procedimento de decisão(algum algoritmo que faz uma sequência de perguntas relacionadas a sim / não ) produz a resposta SIM.

_Perguntas típicas feitas sobre tais formalismos incluem_:

- Qual é o seu poder de expressão?(Pode formalismo X descrever todas as linguas que o formalismo Y pode descrever?)

- Qual a sua reconhecidabilidade? ( Quão difícil é decidir se uma determinada palavra pertence a uma linguagem descrita pelo formalismo X?)
  
  - Qual a comparabilidade?(Quão difícil se uma determinada palavra pertence a uma linguagem descrita no formalismo X e um Y no formalismo, ou em X novamente, são na verdade a mesma língua?)
  
 Surpreendentemente, muitas vezes, as respostas a esses problemas de decisão é "não pode ser feito de modo algum", ou "é estremamente custoso" (com uma caracterização de o quão custoso). Portanto, a teoria da linguagem formal, é uma grande área de aplicação da teoria da computabilidade e teoria da complexidade. Linguagens formais podem ser classificadas na hierarquia de Chomky com base no poder expressivo de sua gramática geradora, bem como a complexidade de seu autômato reconhecedor. Gramáticas livres de contexto e gramáticas, regulares oferecem um bom compromisso entre expressividade e facilidade de análise, e são amplamente utilizadas em aplicações práticas.
  
  ## Operações em linguagens
  
  Certas operações em linguagens são comuns. Isto inclui as operações padraão de conjuntos, tais como união, interseção e complemento. Outra classe de operação é a aplicação _element-wise_ de operações de cadeia.
  
  Exemplos:            
	suponha que L¹ e L² são linguagens sobre algum alfabeto comum;
- A concatenação L¹L² consiste de todas as cadeias da forma _vw_ onde _v_ é uma sequência de L¹ e _w_ uma sequencia de L².
- A interseção L¹ ⋂ L² de L¹ e L² consiste de todas as cadeias que estão contidas em ambas as linguagens.
- O complemento ¬L de uma linguagem em relação a um alfabeto consiste de todas as cadeias sobre o alfabeto que não estão na língua 
- O fecho de Kleene: a linguagem constituída de todas as palavras que concatenações de 0 ou mais palavras na língua original;
- Reversao:
	- Seja a palavra vazia, então eR= e, 
	- para cada palavra não vazia w = X¹...Xn sobre um alfabeto, seja wR = xn..x1,
	- então, para uma palavra L formal, LR = {WR | W ∊ L }.
- Homomorfismo de cadeia
Tais operações de cadeia são usadas para investigar as propriedades de fechamento das classes da linguagem. A classe das linguagens é fechada sob uma operação em particular quando a operação,  aplicada as linguagens na classe, sempre produz uma linguagem na mesma classe. Por exemplo, as linguagens livres de contexto são conhecidas por serem fechadas sob união, concatenação e interseção com linguagens regulares, mas não fechadas sob interseção ou complemento. A teoria dos trios e familiares abstratas de linguagens estuda as propriedades mais comuns de fechamento de famílias de linguagens em seu próprio direito.

# Aplicações 
----
## Linguagens de programação
Um compilador tem geralmente dois componentes distintos. Um analisador léxico, gerado por uma ferramenta como lex, que identifica as os símbolos da gramática da linguagem de programação, por exemplo, identificadores ou palavras-chave, que são eles próprios expressões regulares. No nível conceitual mais básico, um parser, geralmente gerado por um gerador como yacc, tentar decidir se o programa fonte é válido, isto é, se ele pertence á linguagem á linguagem de programação para que o compilador foi construído. Claro, compiladores fazem mais do que apenas analisar o código fonte, eles geralmente o traduzem em algum formato executável. Devido a isso, um analisador geralmente produz mais do que do que uma resposta sim/não, normalmente uma árvore de sintaxe abstrata, que é usado por etapas subsequentes do compilador para, eventualmente, gerar um executável contendo código de máquina que roda diretamente no hardware, ou algum código intermediário que requer uma maquina virtual para executar.

## Teorias formais, sistemas e provas
Na lógica matemática, uma teoria formal é um conjunto de sentenças expressas em uma linguagem formal.

Um sistema formal( também chamado de cálculo lógico, ou um sistema lógico ) é construído por uma linguagem formal, juntamente com um sistema dedutivo (também denominado aparato dedutivo). O sistema dedutivo pode consistir num conjunto de axiomas, ou tem ambos. Embora uma linguagem formal possa ser identificada com suas fórmulas, um sistema convencional pode não ser igualmente identificado pelos seus teoremas. Dois sistemas formais SF e SF' podem ter os mesmos teoremas e ainda diferirem em alguma significante prova teórica (uma fórmula sintática B em um, mas não no outro, por exemplo ).

A prova formal ou derivação é uma sequência finita de fórmulas bem formadas (que pode ser interpretada como proposições), cada um dos  quais é uma axioma ou resulta das fórmulas anteriores na sequência de uma regra de inferência. A última frase da sequência é um teorema de um sistema formal. Provas formais são úteis porque os seus teoremas podem ser interpretadas como proposições verdadeiras.

### Interpretações e modelos 

Linguagens formais são totalmente sintáticas por natureza, mas podem ser semânticas que dão sentido aos elementos da linguagem. Por exemplo, em matemática lógica, o conjunto de possíveis fórmulas de uma lógica particular é uma linguagem formal, e uma interpretação atribui um significado para cada umas das fórmulas, geralmente, um valor verdade.

O estudo das interpretações de linguagens formais é chamado de semântica formal. Na lógica matemática, esta é muitas vezes feito em termos de teoria dos modelos. Em teoria dos modelos, os termos que ocorrem em uma fórmula são interpretados como estruturas matemáticas, e regras fixas de interpretação de composição determinam como o valor verdade da fórmula pode ser derivado da interpretação de seus termos; um modelo para uma fórmula é uma interpretação de formas tais que a fórmula se torne verdade.


#### Artigo original
Linguagem formal [https://pt.wikipedia.org/wiki/Linguagem_formal]


