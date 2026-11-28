| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.077x | 1.036x | 1.587x |
| materialize | 1.006x | 0.989x | 1.229x |
| perft_bulk_d4 | 1.051x | 1.024x | 1.377x |
| perft_full_d3 | 1.328x | 1.109x | 1.620x |
| fen_parse | 0.914x | 0.833x | 1.099x |
| fen_write_native_ep | 1.047x | 1.000x | 1.472x |
| snapshot | 0.986x | 0.805x | 1.440x |
| san_parse_all | 1.022x | 1.014x | 1.800x |
| san_render_native | 1.014x | 0.928x | 2.323x |
| apply_children_full | 0.995x | 0.958x | 1.343x |
| make_unmake_full | 1.059x | 0.955x | 1.289x |
| startpos | 1.011x | 1.011x | 1.246x |
| chess960_materialize | 0.969x | 0.931x | 1.454x |
| chess960_perft_d3 | 1.011x | 1.007x | 1.504x |
| polyglot_key | 1.034x | 1.027x | 35.634x |
| polyglot_recompute | 1.642x | 1.397x | 1.578x |
| repetition_key | 0.959x | 0.922x | 25.693x |
| corpus_count | 1.097x | 1.055x | 1.281x |
| corpus_materialize | 1.014x | 0.989x | 1.128x |
| corpus_uci_sorted | 1.009x | 1.000x | 1.685x |
| replay_trusted_native | 1.062x | 1.049x | 0.778x |
| replay_checked_native | 1.009x | 1.008x | 1.231x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.17x).
