| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.153x | 0.880x | 1.535x |
| materialize | 1.059x | 0.891x | 1.137x |
| perft_bulk_d4 | 1.124x | 0.976x | 1.246x |
| perft_full_d3 | 1.191x | 0.998x | 1.415x |
| fen_parse | 1.095x | 0.943x | 1.214x |
| fen_write_native_ep | 1.031x | 0.931x | 1.722x |
| snapshot | 1.099x | 0.912x | 1.044x |
| san_parse_all | 1.079x | 0.959x | 1.746x |
| san_render_native | 1.052x | 0.933x | 2.813x |
| apply_children_full | 1.148x | 0.905x | 1.164x |
| make_unmake_full | 1.051x | 0.982x | 1.216x |
| startpos | 1.304x | 1.304x | 1.053x |
| chess960_materialize | 1.145x | 1.043x | 1.539x |
| chess960_perft_d3 | 1.263x | 1.214x | 1.588x |
| polyglot_key | 1.115x | 0.999x | 38.904x |
| polyglot_recompute | 1.093x | 1.041x | 1.050x |
| repetition_key | 1.030x | 1.014x | 17.407x |
| corpus_count | 1.236x | 1.135x | 1.270x |
| corpus_materialize | 1.025x | 0.967x | 1.118x |
| corpus_uci_sorted | 1.119x | 1.092x | 1.363x |
| replay_trusted_native | 1.119x | 0.986x | 0.714x |
| replay_checked_native | 1.112x | 1.057x | 1.224x |

Cold first legal move list: 0.010 ms -> 0.004 ms (2.42x).
