| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.857x | 0.811x | 1.526x |
| materialize | 0.910x | 0.781x | 1.029x |
| perft_bulk_d4 | 1.030x | 0.766x | 1.258x |
| perft_full_d3 | 1.007x | 0.801x | 1.565x |
| fen_parse | 0.912x | 0.730x | 1.086x |
| fen_write_native_ep | 0.955x | 0.746x | 1.650x |
| snapshot | 1.341x | 0.786x | 1.008x |
| san_parse_all | 0.960x | 0.750x | 1.742x |
| san_render_native | 1.065x | 0.887x | 2.776x |
| apply_children_full | 0.889x | 0.665x | 1.102x |
| make_unmake_full | 0.914x | 0.703x | 1.238x |
| startpos | 1.950x | 1.950x | 1.559x |
| chess960_materialize | 1.323x | 1.170x | 1.610x |
| chess960_perft_d3 | 1.319x | 1.094x | 1.491x |
| polyglot_key | 1.256x | 1.243x | 35.857x |
| polyglot_recompute | 1.473x | 1.354x | 0.995x |
| repetition_key | 1.371x | 1.352x | 21.212x |
| corpus_count | 1.192x | 1.173x | 1.029x |
| corpus_materialize | 1.658x | 1.573x | 1.271x |
| corpus_uci_sorted | 1.290x | 1.237x | 1.660x |
| replay_trusted_native | 1.294x | 1.142x | 0.597x |
| replay_checked_native | 1.235x | 1.212x | 1.201x |

Cold first legal move list: 0.006 ms -> 0.009 ms (0.73x).
