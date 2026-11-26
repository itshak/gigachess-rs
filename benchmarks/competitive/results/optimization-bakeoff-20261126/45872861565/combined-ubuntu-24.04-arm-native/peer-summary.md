| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.40x | 1.68x | 1.82x | 2.18x | 2.17x | 30/30 |
| materialize | 1.00x | 1.95x | 1.54x | 1.33x | 2.17x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.58x | 1.45x | 2.02x | 2.07x | 30/30 |
| perft_full_d3 | 1.14x | 1.69x | 1.83x | 1.52x | 2.24x | 2.12x | 30/30 |
| fen_parse | 1.01x | 1.24x | 1.33x | 1.28x | 1.76x | 1.37x | 30/30 |
| fen_write_native_ep | 1.00x | 4.43x | 9.19x | 1.56x | 29.04x | 2.39x | 30/30 |
| snapshot | 0.99x | 1.06x | 1.06x | 1.45x | 1.07x | 0.96x | 24/30 |
| san_parse_all | 0.99x | 1.86x | N/A | 4.35x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.58x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.24x | 1.48x | 4.01x | 1.20x | 1.19x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.59x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.80x | 126.39x | 172.98x | 134.81x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.97x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.14x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.43x | 1.70x | 1.37x | 1.99x | 2.20x | 10/10 |
| corpus_materialize | 0.99x | 1.60x | 1.52x | 1.22x | 1.90x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.10x | 2.88x | 2.08x | 2.83x | 1.45x | 10/10 |
| replay_trusted_native | 0.97x | 0.61x | 1.60x | 1.95x | 2.17x | 2.22x | 8/10 |
| replay_checked_native | 0.98x | 2.27x | 1.11x | 7.43x | 6.66x | 8.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
