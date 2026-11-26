| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.132x | 1.031x | 1.227x |
| materialize | 1.016x | 0.998x | 1.202x |
| perft_bulk_d4 | 1.256x | 1.205x | 1.107x |
| perft_full_d3 | 1.191x | 1.146x | 0.719x |
| fen_parse | 1.029x | 1.017x | 1.226x |
| fen_write_native_ep | 0.989x | 0.984x | 1.054x |
| snapshot | 0.970x | 0.947x | 0.751x |
| san_parse_all | 1.488x | 1.441x | 1.394x |
| san_render_native | 1.496x | 1.397x | 1.716x |
| apply_children_full | 1.141x | 1.106x | 0.746x |
| make_unmake_full | 1.139x | 1.103x | 1.026x |
| startpos | 1.066x | 1.066x | 0.844x |
| chess960_materialize | 1.124x | 1.047x | 1.245x |
| chess960_perft_d3 | 1.270x | 1.169x | 1.068x |
| polyglot_key | 1.004x | 0.993x | 24.122x |
| polyglot_recompute | 1.002x | 0.997x | 1.132x |
| repetition_key | 1.086x | 1.065x | 15.471x |
| corpus_count | 1.114x | 1.076x | 1.346x |
| corpus_materialize | 1.042x | 1.038x | 1.187x |
| corpus_uci_sorted | 1.337x | 1.334x | 0.762x |
| replay_trusted_native | 1.138x | 1.106x | 0.364x |
| replay_checked_native | 1.240x | 1.212x | 0.817x |

Cold first legal move list: 154.112 ms → 0.009 ms (18122.35x).
