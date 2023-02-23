# LibEsrafel
EPR spectral simulation toolbox.

# IO

Il modulo `iof` ti consente di importare ed esportare i dati relativi in maniera semplice e veloce.

## Importare uno spettro da ASCII

``` rust
use libesrafel::io::Spectrum;
let spectrum = Spectrum::from_ascii(content);
```

Questo restituisce uno struct chiamato `Spectrum`, che contiene i seguenti campi:
- `idx`, che porta con sé un vettore di interi contenente gli indici dello spettro. Campo tipicamente poco utile, visto che questi valori possono essere ottenuti facilmente dalla posizione degli elementi nel vettore.
- `fld`, cioè un vettore di floats con tutti i valori del campo magnetico applicato dallo strumento (Field [G])
- `int`, cioè un altro vettore di floats che contiene i valori di intensità rilevati dallo strumento durante lo scan al campo corrispondente.

Questi valori possono essere ulteriormente manipolati con i seguenti metodi:
- `into_json()`, per ottenere una stringa json in cui sono stati serializzati;
- `into_tuple()`, per ottenere una tupla che specchia lo struct, al fine di facilitare la costruzione di bindings per altri linguaggi, ad esempio Python.

## Importare dei parametri da file SIM
Il formato SIM è un formato custom ideato originariamente per il software SimCommander, dal gruppo di ricerca di Pedulli e Lucarini all'Università di Bologna. Poiché questa libreria nasce con l'esplicito intento di mantenere la retrocompatibilità con tutto il lavoro precedente, allora è stato supportato anche questo formato, al fine di garantire una più facile esportazione di vecchi file in eventuali formati più recenti.

Il file SIM codifica per dati relativi alla simulazione effettuata e, cioè, conserva in memoria i seguenti valori:
- Numero di punti dello spettro simulato
- Sweep field
- Radicali simulati

Per ogni radicale, vengono salvati in memoria:
- Ampiezza di riga
- Distanza dal centro
- Carattere di Lorenziana
- Quantità relativa (percentuale)
- Nuclei simulati relativi al radicale

Per ogni nucleo simulato si riportano:
- Costante iperfine (solitamente soggetta a variazioni)
- Quantità di nuclei equivalenti
- Spin nucleare

Grazie a LibEsrafel, tutte queste informazioni possono essere facilmente esportate in JSON con una semplice funzione:

``` rust
use libesrafel::io::SimulationState;
let simstate = SimulationState::from_simfile(content).into_json();
```

In realtà, quello che viene restituito è un `Result`, che va quindi unwrapped per ottenere la stringa effettiva, in caso l'operazione abbia avuto successo.
