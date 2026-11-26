| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.86x | 1.74x | 1.77x | 2.42x | 1.98x | 30/30 |
| materialize | 1.10x | 2.07x | 1.66x | 1.08x | 2.21x | 1.45x | 28/30 |
| perft_bulk_d4 | 1.06x | 4.19x | 1.66x | 1.23x | 1.95x | 1.89x | 29/30 |
| perft_full_d3 | 1.03x | 1.62x | 1.79x | 1.27x | 1.97x | 1.96x | 30/30 |
| fen_parse | 1.04x | 1.37x | 1.64x | 1.44x | 1.87x | 1.45x | 30/30 |
| fen_write_native_ep | 1.00x | 4.56x | 8.92x | 1.69x | 28.61x | 2.09x | 30/30 |
| snapshot | 0.99x | 1.62x | 1.21x | 2.50x | 1.14x | 1.13x | 28/30 |
| san_parse_all | 1.02x | 1.76x | N/A | 3.53x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 11.75x | N/A | 2.99x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.11x | 1.23x | 6.16x | 1.15x | 1.31x | 27/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 1.13x | 1.98x | 91.64x | 157.62x | 167.86x | 161.29x | 5/5 |
| chess960_materialize | 1.16x | 1.72x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.12x | 3.40x | 1.63x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 36.17x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 0.90x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.10x | 20.28x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.20x | 5.59x | 1.74x | 1.11x | 2.10x | 2.39x | 9/10 |
| corpus_materialize | 1.09x | 1.40x | 1.28x | 1.16x | 1.90x | 1.31x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.92x | 2.55x | 1.96x | 2.53x | 1.40x | 10/10 |
| replay_trusted_native | 1.14x | 0.65x | 1.66x | 1.88x | 1.84x | 1.83x | 8/10 |
| replay_checked_native | 1.29x | 2.95x | 1.32x | 9.97x | 8.71x | 10.65x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
