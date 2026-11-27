| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.039x | 1.020x | 1.600x |
| materialize | 1.028x | 1.004x | 1.205x |
| perft_bulk_d4 | 1.030x | 0.998x | 1.427x |
| perft_full_d3 | 1.328x | 1.046x | 1.599x |
| fen_parse | 0.987x | 0.958x | 1.168x |
| fen_write_native_ep | 1.001x | 0.993x | 1.841x |
| snapshot | 1.910x | 1.884x | 1.249x |
| san_parse_all | 0.999x | 0.990x | 2.211x |
| san_render_native | 1.007x | 0.992x | 2.139x |
| apply_children_full | 0.976x | 0.924x | 1.278x |
| make_unmake_full | 1.014x | 0.992x | 1.235x |
| startpos | 0.998x | 0.998x | 1.200x |
| chess960_materialize | 0.999x | 0.971x | 1.572x |
| chess960_perft_d3 | 1.004x | 0.995x | 1.608x |
| polyglot_key | 0.990x | 0.978x | 20.509x |
| polyglot_recompute | 0.988x | 0.971x | 1.012x |
| repetition_key | 0.998x | 0.997x | 18.865x |
| corpus_count | 1.068x | 1.040x | 1.456x |
| corpus_materialize | 1.024x | 1.008x | 1.271x |
| corpus_uci_sorted | 0.998x | 0.994x | 1.696x |
| replay_trusted_native | 0.988x | 0.988x | 1.186x |
| replay_checked_native | 0.988x | 0.987x | 1.115x |

Cold first legal move list: 0.008 ms -> 0.007 ms (1.13x).
