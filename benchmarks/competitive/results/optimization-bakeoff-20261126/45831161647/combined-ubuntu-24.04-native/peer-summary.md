| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.38x | 1.55x | 1.50x | 2.40x | 1.80x | 30/30 |
| materialize | 1.05x | 2.13x | 1.69x | 1.12x | 2.11x | 1.44x | 29/30 |
| perft_bulk_d4 | 1.00x | 3.87x | 1.48x | 1.22x | 2.34x | 2.00x | 30/30 |
| perft_full_d3 | 1.01x | 1.30x | 1.34x | 1.04x | 1.94x | 1.62x | 28/30 |
| fen_parse | 0.98x | 1.43x | 1.53x | 1.20x | 1.94x | 1.22x | 30/30 |
| fen_write_native_ep | 1.00x | 4.03x | 9.35x | 1.92x | 28.44x | 2.03x | 30/30 |
| snapshot | 1.00x | 1.89x | 1.25x | 3.82x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.32x | N/A | 3.88x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.28x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.03x | 1.33x | 4.86x | 1.54x | 1.76x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 107.26x | 141.18x | 208.94x | 140.67x | 5/5 |
| chess960_materialize | 1.01x | 1.81x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.87x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.34x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.95x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.02x | 1.62x | 1.20x | 2.17x | 1.88x | 9/10 |
| corpus_materialize | 1.00x | 1.81x | 1.70x | 1.20x | 2.73x | 1.53x | 10/10 |
| corpus_uci_sorted | 1.04x | 2.36x | 3.19x | 2.35x | 2.97x | 1.68x | 10/10 |
| replay_trusted_native | 0.95x | 1.11x | 1.90x | 2.18x | 3.70x | 4.50x | 10/10 |
| replay_checked_native | 1.00x | 2.69x | 1.02x | 5.92x | 6.92x | 7.99x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
