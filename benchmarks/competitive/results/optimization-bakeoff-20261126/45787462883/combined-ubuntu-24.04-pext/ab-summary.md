| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.998x | 1.441x |
| materialize | 0.992x | 0.959x | 1.152x |
| perft_bulk_d4 | 0.784x | 0.704x | 1.037x |
| perft_full_d3 | 0.921x | 0.859x | 0.910x |
| fen_parse | 0.956x | 0.917x | 1.046x |
| fen_write_native_ep | 1.016x | 1.000x | 1.857x |
| snapshot | 1.221x | 1.201x | 1.096x |
| san_parse_all | 0.926x | 0.847x | 2.083x |
| san_render_native | 0.849x | 0.740x | 1.801x |
| apply_children_full | 0.685x | 0.582x | 0.712x |
| make_unmake_full | 0.631x | 0.530x | 0.735x |
| startpos | 1.094x | 1.094x | 1.182x |
| chess960_materialize | 0.983x | 0.971x | 1.450x |
| chess960_perft_d3 | 0.851x | 0.772x | 1.271x |
| polyglot_key | 0.997x | 0.994x | 20.228x |
| polyglot_recompute | 1.046x | 1.034x | 1.022x |
| repetition_key | 1.000x | 0.992x | 18.937x |
| corpus_count | 1.003x | 0.998x | 1.466x |
| corpus_materialize | 1.012x | 1.009x | 1.282x |
| corpus_uci_sorted | 1.026x | 1.021x | 1.724x |
| replay_trusted_native | 0.768x | 0.624x | 0.581x |
| replay_checked_native | 0.837x | 0.779x | 0.791x |

Cold first legal move list: 0.011 ms -> 0.012 ms (0.90x).
