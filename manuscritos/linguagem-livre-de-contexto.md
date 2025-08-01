Na [[Linguagem livre de contexto| teoria das linguagens formais]], uma linguagem livre de contexto (*LLC*) é uma  [[Linguagem Formal|linguagem]] gerada por alguma gramática livre de contexto(*GLC*).
Diferentes gramáticas livres de contexto podem gerar a mesma linguagem livre de contexto, ou, inversamente, uma linguagem livre de contexto pode ser gerado por diferentes gramáticas livres de contexto. É importante distinguir as propriedades da linguagem(propriedades intrínsecas) de propriedades de uma gramática especifica(propriedades extrínsecas).

O conjunto de todas as linguagens livres de contexto é idêntico ao conjunto de linguagens aceitas por um autômato de pilha, o que faz com que essas linguagens sejam passíveis de análise. Na verdade, dada uma GLC, há uma maneira direta de produzir um autômato com pilha para gramática ( e linguagem correspondente), mas indo para outro lado ( produzindo uma gramática dado um autômato) não é tão direta.

## Aplicações

Tais linguagens são importantes para definir [[https://pt.wikipedia.org/wiki/Linguagem_de_programa%C3%A7%C3%A3o|linguagens de programação]].
Por exemplo, as linguagens que requerem o balanceamento de parênteses são geradas pela gramática _S -> SS |(S)| λ_. Da mesma forma, a maioria das expressões aritméticas é gerada por gramaticas livres de contexto.

## Exemplos 

Uma linguagem livre de contexto é _$L=\{a^n b^n : n \geq 1\}$, a linguagem de todas as [[https://pt.wikipedia.org/wiki/Cadeia_de_caracteres|cadeias de caracteres]] não vazias de tamanho par, a primeira metade sempre preenchida com "a"s e a segunda metade sempre preenchida com "b"s. $L$  é gerada pela gramática $S -> aSb|ab$,e é aceita pelo autômato de pilha $M = (\{q_0, q_1, q_f\},\{a,b\},\{a,z\},\delta, q_0, \{q_f\})$, em que $\delta$  é definido da seguinte forma:
$$
\begin{aligned}
\delta(q_0, a, z) &= (q_0, a) \\
\delta(q_0, a, a) &= (q_0, a) \\
\delta(q_0, b, a) &= (q_1, x) \\
\delta(q_1, b, a) &= (q_1, x) \\
\delta(q_1, b, z) &= (q_f, z) \\
\\
\delta(estado,leitura,desempilha) &= (estado)_2,empilha)
\end{aligned}
$$

Onde _z_ é o simbolo inicial e x significa desempilhar.

LLCs não ambíguas são um subconjunto próprio de todos os LLCs: há LLCs inerentemente ambíguas. Um exemplo de uma LLCs inerentemente ambígua é a união de $\{a^n, b^m, c^m, d^m | n,m > 0 \}$ com $\{a^n, b^n, c^m, d^m | n,m > 0 \}$. Este conjunto é livre de contexto, uma vez que a união de duas linguagens livres de contexto é sempre livre de contexto. Mas não há nenhuma maneira de transformar de forma não ambígua cadeias no subconjunto ( não-livre-contexto) $\{ a^n, b^m, c^n, d^n | n > 0 \}$ que é a interseção dessas duas linguagens. 

## Linguagens não livres de contextos 
---
O conjunto $\{ a^n b^n c^n d^n | n > 0 \}$ é uma linguagem sensível ao contexto, mas não existe uma gramática livre de contexto gerando essa linguagem.
Então existem linguagens sensíveis ao contexto que não são livres de contexto. Para  provar que é uma determinada linguagem não é livre de contexto, pode-se empregar o [[https://pt.wikipedia.org/wiki/Lema_do_bombeamento_para_linguagens_livres_de_contexto|Lema do bombeamento para linguagens livres de contexto]] ou uma série de outros métodos, como o [[https://pt.wikipedia.org/wiki/Lema_de_Ogden|Lema de Ogden]] ou Teorema de Parikh.

## Propriedades de fechamento
___
Linguagens livres de contextos são fechadas nas seguintes operações. Se $L$ e $P$ são linguagens livres de contexto e $D$ é uma [[https://pt.wikipedia.org/wiki/Linguagem_regular|linguagem regular]], as seguintes linguagens também são livres de contexto:
- o fecho de Kleene $L^*$ de L
- a imagem $\Phi(L)$ de L sob uma [[https://pt.wikipedia.org/wiki/Homomorfismo|homomorfismo]] $\Phi$
- a concatenação $L$ o $P$ de _L_ e _P_
- a união $L \cup P$ de _L_ _P_
- a interseção (com uma linguagem regular) $L \cup D$ de _L_ e _D_

Linguagens livres de contexto não são fechadas sob complemento, interseção ou diferença. No entanto, se _L_ é uma linguagem livre de contexto e _D_ é uma linguagem regular, então tanto a sua interseção $L \cap D$ e sua diferença L \ D  são linguagens livres de contexto.

## Não fechamento de interseção e complemento

As linguagens livres de contexto não são fechadas sob interseção. Isto pode ser visto tomando as linguagens $A = \{a^n b^n c^m |m,n \geq 0 \}$ e $B = \{ a^m b^n c^n | m,n \geq 0 \}$, são ambos livre de contexto. A interseção de $A \cap B = \{ a^n b^n c^n | n \geq 0 \}$, que pode ser mostrado como sendo não livre do contexto pelo [[https://pt.wikipedia.org/wiki/Lema_do_bombeamento_para_linguagens_livres_de_contexto|Lema do bombeamento para linguagens livres de contexto]].

Linguagens livres de contexto também não estão fechados sob complementação, como para qualquer linguagem de A e B:
$A \cap B = \overline{\,\overline{A} \cup \overline{B}}$ .

# Propriedades de decisão
___
Os seguintes problemas são indecidíveis para quaisquer gramáticas livres de contexto A e B:

- Equivalência: Dadas duas gramáticas livres de contexto A e B é $L(A) = L(B)$?
- Interseção vazia: Dadas duas gramáticas livres de contexto A e B, é $L(A)\cap L(B) = \emptyset$  ? No entanto, a interseção entre uma linguagem livre de contexto e uma linguagem regular é livre de contexto, e a variante do problema onde B é uma gramática regular, é decidível.
- Concatenação: Dada uma gramática livre de contexto _A_ é *$L(A) \subseteq L(B)$? Mais uma vez, a variante do problema em que B é uma gramática regular decidível.
- Universalidade: Dada uma gramática livre de contexto A, é $L(A) = \sum^*$ ?

Os seguintes problemas são decidíveis para quaisquer linguagens livres de contexto:
- Vacuidade: Da uma gramática livre de contexto A, $L(A) = \emptyset$ ?
- Finitude: Dada uma gramática livre de contexto A, $L(A)$ é finito?
- Composição: Dada uma gramática livre de contexto G, e uma palavra $w \in L(G)$? Algoritmos eficientes em tempo polinomial para o problema de composição são o [[https://pt.wikipedia.org/wiki/Algoritmo_CYK|Algoritmo CYK]] e [[https://pt.wikipedia.org/wiki/Algoritmo_Earley|Algoritmo Earley]].

## Análise sintática
---
Determinar uma instância do problema da composição, ou seja, dada uma cadeia $w$, determinar se $w \in L(G)$  onde L é a linguagem gerada por alguma gramática $G$, também é conhecida como [[https://pt.wikipedia.org/wiki/An%C3%A1lise_sint%C3%A1tica_(computa%C3%A7%C3%A3o)|Análise sintática (computação)]].

Formalmente, o conjunto de todas as linguagens livres de contexto é idêntico ao conjunto de linguagens aceitas por [[https://pt.wikipedia.org/wiki/Aut%C3%B4mato_com_pilha|autômato de pilha]](AP). Algoritmos de análise sintática para linguagens livres de cotexto incluem o  [[https://pt.wikipedia.org/wiki/Algoritmo_CYK|Algoritmo CYK]] e o [[https://pt.wikipedia.org/wiki/Algoritmo_Earley|Algoritmo Earley]].

Uma subclasse especial de linguagens livres de contextos é a [[https://pt.wikipedia.org/wiki/Linguagem_livre_de_contexto_determin%C3%ADstica|Linguagem livre de contexto determinística]], que é definida como o conjunto de linguagens aceitas por um [[https://pt.wikipedia.org/wiki/Aut%C3%B4mato_com_pilha_determin%C3%ADstico|Autômato de pilha determinístico]] e pode ser analisado por [[https://pt.wikipedia.org/wiki/Analisador_sint%C3%A1tico_LR| Analisador sintático LR]].

# Determinando se uma linguagem é livre de contexto
---
## **Teorema da iteração para linguagens livre de contexto**

Se $L$ é uma linguagem livre de contexto, então existe um $n$ tal que para todo $s \in L$ tal que $|s| \geq n$,  
$s$ pode ser reescrito da forma $uvxyz$, com $|vxy| \leq n$, $|vy| > 0$, tal que:

$$
\forall i \geq 0,\ uv^i x y^i z \in L
$$
