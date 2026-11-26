| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 3.90x | 1.65x | 1.37x | 1.94x | 1.72x | 29/30 |
| materialize | 0.96x | 1.94x | 1.33x | 1.47x | 2.24x | 1.54x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.26x | 1.36x | 1.01x | 1.80x | 1.57x | 27/30 |
| perft_full_d3 | 0.93x | 1.58x | 1.78x | 1.57x | 2.18x | 1.84x | 30/30 |
| fen_parse | 1.07x | 1.48x | 1.57x | 1.43x | 1.88x | 1.64x | 30/30 |
| fen_write_native_ep | 0.98x | 4.12x | 8.52x | 1.66x | 32.20x | 2.25x | 30/30 |
| snapshot | 0.85x | 1.95x | 0.97x | 4.70x | 0.97x | 1.10x | 23/30 |
| san_parse_all | 0.90x | 1.63x | N/A | 4.76x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.35x | N/A | 4.44x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 2.00x | 1.32x | 10.24x | 1.34x | 1.61x | 28/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.20x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.00x | 78.37x | 158.91x | 238.17x | 174.93x | 5/5 |
| chess960_materialize | 0.95x | 1.41x | 1.23x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.36x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 40.50x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 27.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.98x | 1.65x | 0.81x | 1.79x | 1.75x | 8/10 |
| corpus_materialize | 1.01x | 1.38x | 1.16x | 1.02x | 1.69x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.08x | 1.83x | 2.20x | 1.86x | 2.17x | 1.53x | 10/10 |
| replay_trusted_native | 0.99x | 0.84x | 1.57x | 1.62x | 2.17x | 2.49x | 9/10 |
| replay_checked_native | 1.01x | 2.14x | 1.09x | 8.58x | 8.14x | 9.66x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
