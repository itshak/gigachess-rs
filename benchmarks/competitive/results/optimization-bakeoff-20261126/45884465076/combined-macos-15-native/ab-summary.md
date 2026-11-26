| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.139x | 0.726x | 1.446x |
| materialize | 1.230x | 0.953x | 1.051x |
| perft_bulk_d4 | 1.117x | 0.871x | 1.212x |
| perft_full_d3 | 1.159x | 0.940x | 1.331x |
| fen_parse | 1.055x | 0.832x | 1.199x |
| fen_write_native_ep | 1.134x | 0.838x | 1.684x |
| snapshot | 0.933x | 0.641x | 0.886x |
| san_parse_all | 1.119x | 0.904x | 1.681x |
| san_render_native | 1.114x | 0.924x | 2.934x |
| apply_children_full | 1.002x | 0.702x | 1.151x |
| make_unmake_full | 1.045x | 0.669x | 1.282x |
| startpos | 0.860x | 0.860x | 1.689x |
| chess960_materialize | 0.716x | 0.690x | 1.435x |
| chess960_perft_d3 | 0.784x | 0.703x | 1.425x |
| polyglot_key | 0.791x | 0.684x | 38.318x |
| polyglot_recompute | 0.694x | 0.589x | 0.862x |
| repetition_key | 0.788x | 0.774x | 22.428x |
| corpus_count | 0.853x | 0.806x | 1.317x |
| corpus_materialize | 0.857x | 0.835x | 1.169x |
| corpus_uci_sorted | 0.941x | 0.769x | 1.354x |
| replay_trusted_native | 0.874x | 0.816x | 0.674x |
| replay_checked_native | 0.993x | 0.955x | 1.279x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.80x).
