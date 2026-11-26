| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.098x | 0.798x | 1.532x |
| materialize | 1.001x | 0.871x | 1.142x |
| perft_bulk_d4 | 0.922x | 0.835x | 1.220x |
| perft_full_d3 | 1.077x | 0.951x | 1.337x |
| fen_parse | 0.872x | 0.513x | 0.977x |
| fen_write_native_ep | 0.947x | 0.739x | 1.596x |
| snapshot | 1.248x | 1.031x | 0.995x |
| san_parse_all | 0.992x | 0.842x | 1.841x |
| san_render_native | 0.949x | 0.732x | 2.756x |
| apply_children_full | 0.949x | 0.786x | 1.073x |
| make_unmake_full | 0.812x | 0.685x | 1.259x |
| startpos | 1.308x | 1.308x | 1.179x |
| chess960_materialize | 1.222x | 1.036x | 1.333x |
| chess960_perft_d3 | 1.264x | 1.067x | 1.617x |
| polyglot_key | 1.309x | 1.245x | 49.118x |
| polyglot_recompute | 0.787x | 0.672x | 0.764x |
| repetition_key | 0.988x | 0.943x | 19.714x |
| corpus_count | 1.304x | 1.210x | 1.030x |
| corpus_materialize | 1.321x | 1.293x | 1.111x |
| corpus_uci_sorted | 1.066x | 1.064x | 1.174x |
| replay_trusted_native | 1.180x | 1.038x | 0.609x |
| replay_checked_native | 1.160x | 1.138x | 1.132x |

Cold first legal move list: 0.008 ms -> 0.004 ms (1.82x).
