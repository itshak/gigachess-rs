| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.91x | 4.52x | 1.73x | 1.53x | 2.33x | 1.98x | 30/30 |
| materialize | 0.92x | 1.93x | 1.31x | 1.19x | 2.25x | 1.58x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.04x | 1.60x | 1.43x | 2.29x | 2.10x | 30/30 |
| perft_full_d3 | 0.96x | 1.94x | 1.44x | 1.33x | 1.99x | 1.86x | 30/30 |
| fen_parse | 0.89x | 1.12x | 1.32x | 1.22x | 2.01x | 1.47x | 30/30 |
| fen_write_native_ep | 0.95x | 4.13x | 8.08x | 1.56x | 32.13x | 2.01x | 30/30 |
| snapshot | 0.96x | 1.86x | 0.98x | 4.16x | 1.09x | 1.03x | 23/30 |
| san_parse_all | 0.97x | 2.14x | N/A | 5.81x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.62x | N/A | 3.72x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.30x | 1.40x | 9.80x | 1.48x | 1.67x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.38x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.21x | 151.95x | 238.34x | 183.33x | 5/5 |
| chess960_materialize | 1.04x | 1.62x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.91x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 28.04x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 19.85x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 5.65x | 1.87x | 1.43x | 2.60x | 2.50x | 10/10 |
| corpus_materialize | 1.01x | 1.56x | 1.26x | 1.13x | 1.96x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.77x | 2.18x | 1.80x | 2.14x | 1.40x | 10/10 |
| replay_trusted_native | 1.03x | 0.84x | 1.62x | 2.04x | 2.58x | 2.93x | 9/10 |
| replay_checked_native | 1.01x | 2.48x | 1.11x | 8.42x | 8.20x | 9.80x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
