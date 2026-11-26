| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.54x | 1.89x | 1.63x | 2.57x | 1.93x | 30/30 |
| materialize | 1.05x | 2.28x | 1.69x | 1.75x | 2.69x | 1.99x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.09x | 1.60x | 1.20x | 2.38x | 2.01x | 29/30 |
| perft_full_d3 | 0.99x | 2.03x | 1.83x | 1.43x | 1.89x | 1.55x | 30/30 |
| fen_parse | 1.05x | 1.32x | 1.54x | 1.06x | 1.43x | 1.11x | 26/30 |
| fen_write_native_ep | 0.99x | 4.58x | 10.08x | 2.12x | 27.85x | 2.61x | 30/30 |
| snapshot | 0.99x | 1.99x | 0.99x | 4.64x | 1.00x | 1.33x | 22/30 |
| san_parse_all | 0.99x | 1.91x | N/A | 4.41x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 8.85x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.14x | 1.34x | 4.04x | 1.43x | 1.64x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.09x | N/A | N/A | 5/6 |
| startpos | 0.90x | 1.00x | 78.38x | 137.12x | 169.92x | 140.28x | 5/5 |
| chess960_materialize | 0.98x | 1.75x | 1.70x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.52x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.96x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.10x | 1.11x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.19x | 24.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 5.52x | 2.02x | 1.14x | 2.54x | 2.28x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.31x | 1.09x | 1.75x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.29x | 2.94x | 2.28x | 2.87x | 1.58x | 10/10 |
| replay_trusted_native | 1.00x | 0.90x | 1.66x | 1.57x | 2.23x | 2.54x | 9/10 |
| replay_checked_native | 1.04x | 2.11x | 1.15x | 8.92x | 8.59x | 10.09x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
