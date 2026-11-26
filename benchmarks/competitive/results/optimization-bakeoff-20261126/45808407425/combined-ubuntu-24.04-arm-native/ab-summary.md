| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.988x | 0.981x | 1.354x |
| materialize | 0.997x | 0.986x | 1.232x |
| perft_bulk_d4 | 0.950x | 0.929x | 1.286x |
| perft_full_d3 | 0.971x | 0.950x | 1.022x |
| fen_parse | 0.959x | 0.941x | 1.193x |
| fen_write_native_ep | 0.991x | 0.986x | 1.574x |
| snapshot | 1.177x | 1.161x | 1.179x |
| san_parse_all | 0.961x | 0.922x | 1.766x |
| san_render_native | 0.838x | 0.760x | 2.137x |
| apply_children_full | 0.946x | 0.878x | 1.058x |
| make_unmake_full | 0.873x | 0.828x | 1.262x |
| startpos | 0.997x | 0.997x | 1.214x |
| chess960_materialize | 0.994x | 0.993x | 1.326x |
| chess960_perft_d3 | 0.963x | 0.947x | 1.292x |
| polyglot_key | 1.040x | 0.940x | 26.080x |
| polyglot_recompute | 0.986x | 0.983x | 1.055x |
| repetition_key | 1.154x | 1.091x | 21.434x |
| corpus_count | 1.041x | 1.034x | 1.557x |
| corpus_materialize | 1.022x | 1.015x | 1.217x |
| corpus_uci_sorted | 1.008x | 1.006x | 1.462x |
| replay_trusted_native | 0.876x | 0.865x | 0.455x |
| replay_checked_native | 0.919x | 0.910x | 0.896x |

Cold first legal move list: 0.011 ms -> 0.011 ms (1.01x).
