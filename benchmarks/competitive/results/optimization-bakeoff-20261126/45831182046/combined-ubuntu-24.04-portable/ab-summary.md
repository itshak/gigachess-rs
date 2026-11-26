| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 0.951x | 1.221x |
| materialize | 1.009x | 0.974x | 1.654x |
| perft_bulk_d4 | 1.037x | 0.991x | 1.109x |
| perft_full_d3 | 1.013x | 0.987x | 1.455x |
| fen_parse | 0.999x | 0.974x | 1.060x |
| fen_write_native_ep | 0.997x | 0.982x | 1.859x |
| snapshot | 1.002x | 0.992x | 1.002x |
| san_parse_all | 0.975x | 0.963x | 1.900x |
| san_render_native | 1.021x | 1.013x | 2.158x |
| apply_children_full | 1.031x | 1.000x | 1.358x |
| make_unmake_full | 1.013x | 0.980x | 1.251x |
| startpos | 1.001x | 1.001x | 1.353x |
| chess960_materialize | 1.077x | 1.014x | 1.542x |
| chess960_perft_d3 | 1.044x | 0.996x | 1.436x |
| polyglot_key | 0.988x | 0.976x | 43.408x |
| polyglot_recompute | 1.019x | 1.017x | 1.027x |
| repetition_key | 1.234x | 1.124x | 25.588x |
| corpus_count | 0.995x | 0.955x | 0.943x |
| corpus_materialize | 1.029x | 1.017x | 1.127x |
| corpus_uci_sorted | 1.022x | 1.017x | 1.553x |
| replay_trusted_native | 1.008x | 0.998x | 0.875x |
| replay_checked_native | 1.084x | 1.035x | 1.186x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.99x).
