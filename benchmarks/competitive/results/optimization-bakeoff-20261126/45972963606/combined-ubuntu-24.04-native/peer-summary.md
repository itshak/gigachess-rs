| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.82x | 1.77x | 1.69x | 2.66x | 2.05x | 30/30 |
| materialize | 0.98x | 2.03x | 1.71x | 1.17x | 2.18x | 1.47x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.28x | 1.69x | 1.38x | 2.66x | 2.29x | 30/30 |
| perft_full_d3 | 1.36x | 2.04x | 2.12x | 1.61x | 3.05x | 2.55x | 30/30 |
| fen_parse | 0.78x | 1.11x | 1.22x | 0.90x | 1.76x | 1.11x | 25/30 |
| fen_write_native_ep | 0.96x | 3.85x | 9.33x | 1.77x | 26.21x | 2.03x | 30/30 |
| snapshot | 1.00x | 1.91x | 1.25x | 3.69x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.28x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 10.46x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.01x | 1.33x | 4.77x | 1.52x | 1.75x | 30/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 99.23x | 134.38x | 237.39x | 169.07x | 5/5 |
| chess960_materialize | 1.03x | 1.94x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.13x | 1.62x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 19.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 4.68x | 1.90x | 1.39x | 2.53x | 2.22x | 10/10 |
| corpus_materialize | 1.00x | 1.92x | 1.82x | 1.27x | 2.85x | 1.60x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.32x | 3.13x | 2.34x | 2.94x | 1.67x | 10/10 |
| replay_trusted_native | 0.96x | 1.05x | 1.87x | 2.06x | 3.55x | 4.31x | 9/10 |
| replay_checked_native | 0.99x | 2.86x | 1.12x | 6.19x | 7.09x | 8.41x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
