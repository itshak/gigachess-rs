| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.916x | 1.332x |
| materialize | 0.986x | 0.938x | 1.024x |
| perft_bulk_d4 | 1.000x | 0.900x | 1.408x |
| perft_full_d3 | 0.945x | 0.879x | 1.365x |
| fen_parse | 0.965x | 0.710x | 1.253x |
| fen_write_native_ep | 0.989x | 0.829x | 1.324x |
| snapshot | 0.985x | 0.837x | 0.981x |
| san_parse_all | 0.884x | 0.626x | 1.791x |
| san_render_native | 0.928x | 0.748x | 2.585x |
| apply_children_full | 0.893x | 0.769x | 0.961x |
| make_unmake_full | 0.943x | 0.889x | 1.247x |
| startpos | 0.783x | 0.783x | 1.152x |
| chess960_materialize | 0.982x | 0.958x | 1.473x |
| chess960_perft_d3 | 1.015x | 0.975x | 1.580x |
| polyglot_key | 1.203x | 0.981x | 41.046x |
| polyglot_recompute | 1.046x | 1.009x | 1.023x |
| repetition_key | 1.127x | 1.007x | 20.067x |
| corpus_count | 1.036x | 0.878x | 1.237x |
| corpus_materialize | 1.165x | 1.102x | 1.168x |
| corpus_uci_sorted | 0.992x | 0.982x | 1.368x |
| replay_trusted_native | 1.053x | 1.001x | 0.574x |
| replay_checked_native | 1.019x | 0.972x | 0.948x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.96x).
