| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.67x | 1.96x | 1.70x | 2.68x | 2.00x | 30/30 |
| materialize | 0.94x | 2.15x | 1.59x | 1.76x | 2.49x | 1.90x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.03x | 1.61x | 1.26x | 2.50x | 1.96x | 29/30 |
| perft_full_d3 | 1.24x | 1.69x | 1.97x | 1.87x | 2.48x | 2.05x | 30/30 |
| fen_parse | 0.99x | 1.26x | 1.55x | 1.17x | 1.44x | 1.11x | 30/30 |
| fen_write_native_ep | 0.99x | 4.60x | 10.10x | 1.95x | 28.17x | 2.22x | 30/30 |
| snapshot | 1.00x | 1.66x | 1.33x | 3.99x | 1.33x | 1.34x | 30/30 |
| san_parse_all | 1.02x | 1.73x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 8.98x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.14x | 1.40x | 4.11x | 1.33x | 1.72x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 0.89x | 0.99x | 78.64x | 155.40x | 167.47x | 138.01x | 4/5 |
| chess960_materialize | 1.01x | 1.81x | 1.69x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.53x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.69x | 1.63x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.15x | 22.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.22x | 6.27x | 2.32x | 1.30x | 2.84x | 2.60x | 10/10 |
| corpus_materialize | 1.00x | 1.43x | 1.31x | 1.08x | 1.76x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.32x | 2.98x | 2.25x | 2.86x | 1.57x | 10/10 |
| replay_trusted_native | 1.01x | 0.86x | 1.60x | 1.57x | 2.20x | 2.47x | 9/10 |
| replay_checked_native | 0.99x | 2.19x | 1.15x | 8.99x | 8.78x | 10.14x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
