| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.083x | 0.705x | 1.533x |
| materialize | 1.068x | 0.812x | 1.094x |
| perft_bulk_d4 | 1.206x | 0.814x | 1.344x |
| perft_full_d3 | 1.376x | 0.874x | 1.599x |
| fen_parse | 1.045x | 0.740x | 1.173x |
| fen_write_native_ep | 0.983x | 0.864x | 1.838x |
| snapshot | 1.034x | 0.680x | 0.854x |
| san_parse_all | 1.096x | 0.891x | 1.748x |
| san_render_native | 0.969x | 0.849x | 2.871x |
| apply_children_full | 1.070x | 0.791x | 1.085x |
| make_unmake_full | 0.961x | 0.833x | 1.226x |
| startpos | 1.244x | 1.244x | 1.162x |
| chess960_materialize | 1.262x | 1.239x | 1.761x |
| chess960_perft_d3 | 1.229x | 1.145x | 1.447x |
| polyglot_key | 1.342x | 1.179x | 37.665x |
| polyglot_recompute | 1.123x | 1.123x | 0.949x |
| repetition_key | 1.195x | 1.172x | 19.605x |
| corpus_count | 1.381x | 1.238x | 1.278x |
| corpus_materialize | 1.154x | 1.108x | 1.183x |
| corpus_uci_sorted | 1.047x | 0.973x | 1.494x |
| replay_trusted_native | 1.412x | 1.394x | 0.637x |
| replay_checked_native | 1.043x | 0.938x | 1.213x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.85x).
