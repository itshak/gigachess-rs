| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.12x | 4.64x | 1.90x | 1.73x | 2.28x | 2.18x | 30/30 |
| materialize | 1.03x | 2.04x | 1.27x | 1.39x | 2.44x | 1.57x | 30/30 |
| perft_bulk_d4 | 1.11x | 3.96x | 1.57x | 1.32x | 2.05x | 1.88x | 30/30 |
| perft_full_d3 | 1.08x | 1.59x | 1.65x | 1.55x | 2.14x | 1.94x | 30/30 |
| fen_parse | 1.00x | 1.38x | 1.61x | 1.57x | 2.12x | 1.62x | 30/30 |
| fen_write_native_ep | 0.99x | 4.40x | 8.10x | 1.69x | 34.38x | 2.28x | 30/30 |
| snapshot | 0.96x | 2.21x | 1.16x | 6.09x | 1.17x | 1.15x | 30/30 |
| san_parse_all | 1.04x | 1.91x | N/A | 5.71x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.70x | N/A | 4.34x | N/A | N/A | 12/12 |
| apply_children_full | 1.14x | 2.20x | 1.33x | 11.29x | 1.42x | 1.60x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.38x | N/A | N/A | 6/6 |
| startpos | 1.19x | 1.19x | 90.38x | 214.34x | 268.21x | 211.17x | 5/5 |
| chess960_materialize | 1.04x | 1.72x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.10x | 2.63x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 37.93x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.10x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 21.88x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.30x | 5.86x | 2.33x | 1.35x | 2.57x | 2.58x | 10/10 |
| corpus_materialize | 1.04x | 1.45x | 1.20x | 1.09x | 1.81x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.74x | 2.12x | 1.81x | 2.05x | 1.41x | 10/10 |
| replay_trusted_native | 1.03x | 0.80x | 1.67x | 1.89x | 2.27x | 2.45x | 9/10 |
| replay_checked_native | 1.02x | 2.22x | 1.09x | 8.18x | 7.62x | 9.30x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
