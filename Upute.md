### DIGITALNI POTPIS
#### Osnovi pojmovi
Jedan od osnovnih načina zaštite vjerodostojnosti informacije je potpis.
Potpis treba biti jedinstven i služi prilikom identifikacije i provjere pojedinca.
Kod elektroničke informacije, koncept potpisa se mijenja.
Potpis više ne smije biti jedinstven osobi koja potpisuje i u isto vrijeme nezavisan o
informaciji koja se potpisuje, jer je elektronička kopija potpisa identična
originalu i potpisivanje neovlaštenih dokumenata time postaje trivijalno.

Nezaštićene elektroničke poruke je moguće relativno jednostavno presresti,
neovlašteno čitati i mijenjati. Za digitalno nepotpisani e-mail nemoguće je dokazati izvornost,
a niti integritet, jer se svatko može krivo predstaviti. Sigurnosna infrastruktura, točnije,
**infrastruktura javnog ključa** (PKI - Public Key Infrastructure) je osnova za ostvarenje sigurne
i povjerljive razmjene podataka između sudionika u sustavu te potporu ostvarenju funkcija
identifikacije i autentifikacije sudionika. Uz pomoć PKI postiže se tajnost i integritet informacije.
Nadalje, PKI omogućuje sigurnu autentifikaciju sudionika u komunikaciji,
razmjena dokumenata s mogućnošću kriptiranja, digitalnog potpisivanja
i supotpisivanja te jedinstvenu registraciju javnih ključeva.

**Digitalna omotnica** osigurava tajnost, ali ne i besprijekornost informacije.
Poruku pošiljatelj kriptira proizvoljnim (obično slučajno generiranim) simetričnim
ključem K koristeći neki od simetričnih algoritama kriptiranja. Simetrični (sjednički) ključ
se kriptira javnim ključem primatelja PB. Kriptirana poruka i kriptirani ključ čine digitalnu omotnicu:

digitalna_omotnica = { E(m,K); E(K,PB) }.

**Digitalni potpis.** Od posebne je važnosti digitalni potpis koji uspostavlja identitet sudionika u
elektroničkom poslovanju i osigurava integritet podataka. Drugim riječima, digitalni potpis služi za 
utvrđivanje besprijekornosti informacije i za identifikaciju pošiljatelja. Digitalni potpis ne osigurava 
tajnost informacije. Iz poruke pošiljatelj izračunava **sažetak** (message digest) koristeći hash funkciju 
(hash ili digest function). Sažetak se kriptira privatnim ključem pošiljatelja SA i dodaje se izvornoj poruci:

digitalni_potpis = { m; E[Q(m), SA] },
gdje je Q(m) sažetak poruke m.

Primatelj može utvrditi autentičnost pošiljatelja takve poruke, kao i potvrditi integritet poruke dekriptirajući 
digitalni potpis javnim ključem pošiljatelja i uspoređujući rezultat s izračunatim sažetkom primljene 
poruke dobivenim korištenjem istog matematičkog algoritma kojeg je primijenio i pošiljatelj.
Prema tome, digitalni potpis osigurava autentičnost (identitet pošiljatelja utvrđuje se dešifriranjem
sažetka poruke), integritet (provjerom sažetka poruke utvrđuje se je li se poruka mijenjala na putu
do primatelja) i neporecivost (pošiljatelj ne može poreći sudjelovanje u transakciji jer jedino
on ima pristup do svog privatnog ključa kojim je potpisao poruku).

**Digitalni pečat.** Digitalni pečat je digitalno potpisana digitalna omotnica. 
Digitalnim potpisom nije osigurana tajnost poruke (poruku svatko može pročitati), 
ali su osigurani autentičnost, integritet i neporecivost. S druge strane, digitalnom
omotnicom je osigurana samo tajnost. Digitalni pečat osigurava sva četiri 
sigurnosna zahtijeva: tajnost, autentičnost, integritet i neporecivost.

### ZADATAK

Ostvariti program koji omogućuje:
- [x] stvaranje i pohranjivanje kriptografskih ključeva u datoteke 
tajni_kljuc.txt, javni_kljuc.txt i privatni_kljuc.txt;
- [x] kriptiranje, odnosno dekriptiranje zadane datoteke simetričnim i asimetričnim algoritmom;
- [x] izračunavanje sažetka poruke (ulazne datoteke);
- [x] digitalno potpisivanje ulazne datoteke te provjeru digitalnog potpisa.

Uputa: Za simetrično i asimetrično kriptiranje te za izračunavanje sažetka poruke 
koristiti gotove, slobodno raspoložive izvorne tekstove programa po vlastitom izboru. 
Svaki korak mora biti spremljen u posebnu datoteku npr. kriptirani tekst treba biti spremljen 
u posebnu datoteku, potpis treba biti spremljen u posebnu datoteku, sažetak treba biti spremljen 
u posebnu datoteku. Kod dekriptiranja teksta je potrebno učitati kriptirani tekst iz datoteke. 
Kod provjere digitalnog potpisa je potrebno ponovno učitati digitalni potpis iz datoteke i 
originalnu datoteku koja je bila potpisana. Kod provjere digitalnog potpisa se očekuje da se provjeri 
ispravnost digitalnog potpisa pri čemu do pogreške (promjene sadržaja) može doći u originalnoj datoteci 
ili u datoteci koja sadržava digitalni potpis. Zabranjena je uporaba nesigurnih algoritama DES i MD5.
