| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.979x | 0.849x | 1.297x |
| materialize | 1.022x | 0.884x | 1.087x |
| perft_bulk_d4 | 1.260x | 1.114x | 1.346x |
| perft_full_d3 | 0.978x | 0.917x | 0.635x |
| fen_parse | 1.091x | 1.000x | 1.488x |
| fen_write_native_ep | 0.987x | 0.947x | 1.221x |
| snapshot | 1.339x | 1.275x | 1.034x |
| san_parse_all | 1.985x | 1.923x | 1.754x |
| san_render_native | 1.857x | 1.396x | 3.293x |
| apply_children_full | 1.168x | 1.125x | 0.812x |
| make_unmake_full | 1.030x | 0.949x | 0.901x |
| startpos | 1.246x | 1.246x | 1.082x |
| chess960_materialize | 1.082x | 1.017x | 1.189x |
| chess960_perft_d3 | 1.375x | 1.260x | 1.406x |
| polyglot_key | 0.911x | 0.830x | 20.468x |
| polyglot_recompute | 0.997x | 0.949x | 0.990x |
| repetition_key | 1.224x | 1.138x | 14.997x |
| corpus_count | 0.883x | 0.766x | 1.111x |
| corpus_materialize | 1.039x | 0.997x | 1.177x |
| corpus_uci_sorted | 1.328x | 1.293x | 1.071x |
| replay_trusted_native | 0.949x | 0.927x | 0.477x |
| replay_checked_native | 1.297x | 1.258x | 0.887x |
