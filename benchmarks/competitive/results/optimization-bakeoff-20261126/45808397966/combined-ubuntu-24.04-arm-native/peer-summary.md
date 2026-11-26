| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.07x | 1.55x | 1.66x | 2.04x | 1.99x | 30/30 |
| materialize | 1.05x | 1.92x | 1.50x | 1.31x | 2.12x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.60x | 1.47x | 1.34x | 1.87x | 1.92x | 30/30 |
| perft_full_d3 | 1.04x | 1.22x | 1.33x | 1.10x | 1.62x | 1.53x | 29/30 |
| fen_parse | 0.98x | 1.22x | 1.31x | 1.25x | 1.74x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.61x | 9.27x | 1.59x | 29.60x | 2.51x | 30/30 |
| snapshot | 0.96x | 1.10x | 1.03x | 1.45x | 1.02x | 0.95x | 21/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.68x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.07x | 1.29x | 1.52x | 4.11x | 1.24x | 1.24x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.30x | 76.13x | 134.60x | 177.21x | 144.11x | 5/5 |
| chess960_materialize | 1.09x | 1.67x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.10x | 2.99x | 1.45x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 24.76x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 18.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.84x | 4.03x | 1.55x | 1.28x | 1.82x | 2.01x | 10/10 |
| corpus_materialize | 1.02x | 1.60x | 1.55x | 1.23x | 1.92x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.52x | 1.38x | 1.67x | 1.87x | 1.91x | 8/10 |
| replay_checked_native | 0.99x | 1.99x | 0.97x | 6.52x | 5.84x | 7.58x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
