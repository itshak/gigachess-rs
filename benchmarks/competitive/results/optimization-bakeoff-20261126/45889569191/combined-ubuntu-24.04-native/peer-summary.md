| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.38x | 1.89x | 1.60x | 2.47x | 1.96x | 30/30 |
| materialize | 0.96x | 2.14x | 1.62x | 1.74x | 2.56x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.76x | 1.55x | 1.17x | 2.30x | 1.87x | 29/30 |
| perft_full_d3 | 1.25x | 1.72x | 2.01x | 1.87x | 2.46x | 2.08x | 30/30 |
| fen_parse | 0.97x | 1.37x | 1.55x | 1.12x | 1.65x | 1.25x | 30/30 |
| fen_write_native_ep | 1.00x | 4.64x | 9.99x | 1.92x | 27.69x | 2.54x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.31x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 0.90x | 1.73x | N/A | 3.85x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.03x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.13x | 1.34x | 4.04x | 1.34x | 1.69x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 88.20x | 167.26x | 222.01x | 174.83x | 5/5 |
| chess960_materialize | 1.00x | 1.75x | 1.74x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.45x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 40.88x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.10x | 1.13x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.24x | 24.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.39x | 2.03x | 1.11x | 2.56x | 2.28x | 10/10 |
| corpus_materialize | 1.01x | 1.39x | 1.29x | 1.04x | 1.72x | 1.16x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.21x | 2.86x | 2.23x | 2.74x | 1.54x | 10/10 |
| replay_trusted_native | 0.98x | 0.86x | 1.54x | 1.53x | 2.12x | 2.50x | 9/10 |
| replay_checked_native | 1.01x | 2.22x | 1.22x | 9.19x | 8.94x | 10.60x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
