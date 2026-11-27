| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.90x | 1.64x | 1.43x | 1.91x | 1.37x | 30/30 |
| materialize | 0.91x | 2.10x | 1.70x | 1.80x | 2.59x | 1.91x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.00x | 1.59x | 1.19x | 1.99x | 1.78x | 29/30 |
| perft_full_d3 | 1.01x | 1.36x | 1.58x | 1.41x | 1.92x | 1.60x | 30/30 |
| fen_parse | 1.01x | 1.46x | 1.46x | 1.17x | 1.60x | 1.17x | 30/30 |
| fen_write_native_ep | 1.03x | 3.03x | 9.86x | 1.87x | 26.63x | 2.58x | 30/30 |
| snapshot | 1.00x | 1.74x | 1.25x | 3.24x | 1.24x | 1.00x | 26/30 |
| san_parse_all | 1.00x | 1.94x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.24x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.20x | 1.45x | 4.17x | 1.36x | 1.56x | 30/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.36x | 57.22x | 111.24x | 130.82x | 98.45x | 5/5 |
| chess960_materialize | 0.94x | 1.70x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.12x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 42.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.23x | 25.75x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.10x | 2.03x | 1.13x | 2.08x | 2.82x | 10/10 |
| corpus_materialize | 1.00x | 1.50x | 1.40x | 1.13x | 1.82x | 1.25x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.92x | 2.87x | 2.18x | 2.80x | 1.52x | 10/10 |
| replay_trusted_native | 0.97x | 0.84x | 1.58x | 1.60x | 2.20x | 1.96x | 9/10 |
| replay_checked_native | 0.99x | 2.01x | 1.15x | 8.79x | 8.62x | 9.79x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
