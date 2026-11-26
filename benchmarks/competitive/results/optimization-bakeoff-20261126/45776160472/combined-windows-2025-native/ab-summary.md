| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 0.940x | 1.247x |
| materialize | 1.003x | 0.982x | 1.216x |
| perft_bulk_d4 | 0.998x | 0.955x | 1.288x |
| perft_full_d3 | 0.985x | 0.936x | 0.728x |
| fen_parse | 1.074x | 1.021x | 1.268x |
| fen_write_native_ep | 1.548x | 1.430x | 1.544x |
| snapshot | 0.571x | 0.563x | 0.859x |
| san_parse_all | 1.033x | 1.001x | 1.864x |
| san_render_native | 1.112x | 1.012x | 3.661x |
| apply_children_full | 0.973x | 0.943x | 0.826x |
| make_unmake_full | 1.037x | 0.986x | 0.989x |
| startpos | 0.985x | 0.985x | 1.127x |
| chess960_materialize | 1.160x | 1.084x | 1.188x |
| chess960_perft_d3 | 1.098x | 1.080x | 1.323x |
| polyglot_key | 1.134x | 1.053x | 29.425x |
| polyglot_recompute | 1.002x | 0.992x | 0.987x |
| repetition_key | 1.064x | 1.031x | 19.104x |
| corpus_count | 1.019x | 1.014x | 1.372x |
| corpus_materialize | 1.066x | 1.059x | 1.141x |
| corpus_uci_sorted | 1.033x | 0.995x | 0.957x |
| replay_trusted_native | 1.136x | 1.107x | 0.515x |
| replay_checked_native | 1.128x | 1.045x | 0.886x |

Cold first legal move list: 0.042 ms -> 0.044 ms (0.96x).
