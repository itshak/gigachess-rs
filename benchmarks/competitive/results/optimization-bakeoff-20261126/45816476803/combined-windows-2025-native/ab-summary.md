| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.984x | 0.969x | 1.302x |
| materialize | 0.985x | 0.968x | 1.184x |
| perft_bulk_d4 | 0.986x | 0.972x | 1.251x |
| perft_full_d3 | 1.013x | 0.977x | 1.266x |
| fen_parse | 0.917x | 0.871x | 1.320x |
| fen_write_native_ep | 1.018x | 0.957x | 1.507x |
| snapshot | 0.951x | 0.809x | 1.439x |
| san_parse_all | 0.992x | 0.955x | 1.959x |
| san_render_native | 1.013x | 0.966x | 3.734x |
| apply_children_full | 1.094x | 1.013x | 1.302x |
| make_unmake_full | 0.943x | 0.836x | 1.226x |
| startpos | 0.951x | 0.951x | 1.246x |
| chess960_materialize | 0.958x | 0.952x | 1.331x |
| chess960_perft_d3 | 0.945x | 0.942x | 1.370x |
| polyglot_key | 0.873x | 0.862x | 32.857x |
| polyglot_recompute | 0.955x | 0.929x | 0.977x |
| repetition_key | 0.951x | 0.921x | 22.131x |
| corpus_count | 0.993x | 0.991x | 1.063x |
| corpus_materialize | 1.070x | 1.034x | 1.122x |
| corpus_uci_sorted | 0.988x | 0.983x | 1.388x |
| replay_trusted_native | 1.093x | 1.045x | 0.656x |
| replay_checked_native | 1.013x | 0.995x | 0.974x |

Cold first legal move list: 0.012 ms -> 0.014 ms (0.83x).
