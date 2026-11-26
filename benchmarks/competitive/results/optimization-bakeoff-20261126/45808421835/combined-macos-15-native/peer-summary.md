| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.36x | 1.36x | 1.54x | 2.01x | 1.72x | 28/30 |
| materialize | 1.09x | 1.96x | 1.50x | 1.06x | 2.21x | 1.32x | 27/30 |
| perft_bulk_d4 | 1.07x | 4.46x | 1.58x | 1.37x | 2.18x | 2.00x | 30/30 |
| perft_full_d3 | 1.04x | 1.64x | 1.75x | 1.27x | 1.83x | 1.94x | 29/30 |
| fen_parse | 1.15x | 1.33x | 1.45x | 1.45x | 1.83x | 1.56x | 30/30 |
| fen_write_native_ep | 1.07x | 4.27x | 9.60x | 1.87x | 30.55x | 2.37x | 30/30 |
| snapshot | 1.05x | 1.88x | 1.12x | 2.64x | 1.16x | 1.05x | 28/30 |
| san_parse_all | 0.94x | 1.78x | N/A | 3.80x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.58x | N/A | 2.75x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.01x | 1.09x | 4.91x | 1.08x | 1.15x | 24/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.19x | N/A | N/A | 5/6 |
| startpos | 0.73x | 1.12x | 54.20x | 98.18x | 128.06x | 108.74x | 5/5 |
| chess960_materialize | 0.91x | 1.53x | 1.20x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.12x | 2.98x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.24x | 36.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.35x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.31x | 19.83x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.26x | 5.86x | 2.04x | 1.22x | 2.54x | 2.89x | 10/10 |
| corpus_materialize | 1.17x | 1.59x | 1.50x | 1.20x | 2.00x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.17x | 1.97x | 3.16x | 2.08x | 2.73x | 1.29x | 10/10 |
| replay_trusted_native | 1.23x | 0.70x | 1.45x | 1.37x | 1.71x | 1.51x | 8/10 |
| replay_checked_native | 1.48x | 2.35x | 1.02x | 8.43x | 7.11x | 8.97x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
