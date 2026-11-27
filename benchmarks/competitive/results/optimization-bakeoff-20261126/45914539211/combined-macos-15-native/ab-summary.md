| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.027x | 0.653x | 1.561x |
| materialize | 0.964x | 0.824x | 1.143x |
| perft_bulk_d4 | 1.073x | 0.838x | 1.255x |
| perft_full_d3 | 1.255x | 1.082x | 1.430x |
| fen_parse | 1.111x | 1.002x | 1.205x |
| fen_write_native_ep | 1.232x | 1.064x | 1.767x |
| snapshot | 1.005x | 0.537x | 0.854x |
| san_parse_all | 1.041x | 0.889x | 1.804x |
| san_render_native | 1.117x | 0.928x | 2.841x |
| apply_children_full | 1.076x | 0.898x | 1.127x |
| make_unmake_full | 0.944x | 0.830x | 1.243x |
| startpos | 0.838x | 0.838x | 0.932x |
| chess960_materialize | 0.932x | 0.928x | 1.504x |
| chess960_perft_d3 | 1.040x | 0.957x | 1.489x |
| polyglot_key | 0.954x | 0.853x | 35.828x |
| polyglot_recompute | 1.021x | 0.993x | 1.021x |
| repetition_key | 1.087x | 1.079x | 19.859x |
| corpus_count | 1.061x | 1.020x | 1.396x |
| corpus_materialize | 0.845x | 0.772x | 0.998x |
| corpus_uci_sorted | 1.052x | 0.898x | 1.272x |
| replay_trusted_native | 1.253x | 1.188x | 0.672x |
| replay_checked_native | 0.980x | 0.885x | 1.081x |

Cold first legal move list: 0.009 ms -> 0.004 ms (2.12x).
