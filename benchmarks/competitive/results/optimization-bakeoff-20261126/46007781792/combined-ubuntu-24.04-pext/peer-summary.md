| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.70x | 1.96x | 1.72x | 2.69x | 2.00x | 30/30 |
| materialize | 0.90x | 2.05x | 1.50x | 1.72x | 2.41x | 1.86x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.07x | 1.62x | 1.27x | 2.52x | 1.97x | 29/30 |
| perft_full_d3 | 1.24x | 1.66x | 1.94x | 1.87x | 2.48x | 2.02x | 30/30 |
| fen_parse | 0.94x | 1.21x | 1.28x | 1.10x | 1.36x | 1.18x | 29/30 |
| fen_write_native_ep | 0.99x | 4.89x | 10.18x | 2.03x | 29.11x | 2.16x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.01x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.12x | 1.91x | N/A | 4.37x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 8.97x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.13x | 1.37x | 4.06x | 1.33x | 1.72x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.00x | 79.32x | 144.80x | 171.29x | 157.19x | 5/5 |
| chess960_materialize | 0.86x | 1.52x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.54x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.11x | 22.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.22x | 6.24x | 2.34x | 1.29x | 2.83x | 2.61x | 10/10 |
| corpus_materialize | 1.00x | 1.43x | 1.30x | 1.08x | 1.73x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.33x | 3.07x | 2.24x | 3.03x | 1.56x | 10/10 |
| replay_trusted_native | 1.01x | 0.86x | 1.57x | 1.59x | 2.21x | 2.53x | 9/10 |
| replay_checked_native | 1.01x | 2.22x | 1.16x | 9.25x | 8.90x | 10.47x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
