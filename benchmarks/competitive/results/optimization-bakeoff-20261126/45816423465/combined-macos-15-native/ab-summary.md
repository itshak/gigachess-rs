| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.905x | 0.682x | 1.397x |
| materialize | 0.880x | 0.571x | 1.096x |
| perft_bulk_d4 | 0.759x | 0.537x | 1.051x |
| perft_full_d3 | 0.909x | 0.490x | 1.125x |
| fen_parse | 0.908x | 0.588x | 1.212x |
| fen_write_native_ep | 0.977x | 0.580x | 1.829x |
| snapshot | 0.969x | 0.616x | 1.021x |
| san_parse_all | 0.812x | 0.516x | 1.821x |
| san_render_native | 0.918x | 0.706x | 2.736x |
| apply_children_full | 0.907x | 0.534x | 0.932x |
| make_unmake_full | 0.927x | 0.658x | 1.208x |
| startpos | 0.985x | 0.985x | 1.203x |
| chess960_materialize | 0.910x | 0.886x | 1.369x |
| chess960_perft_d3 | 0.739x | 0.690x | 1.173x |
| polyglot_key | 0.982x | 0.956x | 37.748x |
| polyglot_recompute | 1.011x | 0.888x | 1.065x |
| repetition_key | 0.972x | 0.931x | 19.652x |
| corpus_count | 0.828x | 0.775x | 0.974x |
| corpus_materialize | 0.942x | 0.935x | 1.117x |
| corpus_uci_sorted | 0.966x | 0.947x | 1.453x |
| replay_trusted_native | 1.127x | 1.092x | 0.603x |
| replay_checked_native | 0.731x | 0.681x | 0.910x |

Cold first legal move list: 0.016 ms -> 0.007 ms (2.15x).
