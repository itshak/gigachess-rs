| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.66x | 1.62x | 1.71x | 2.09x | 1.90x | 29/30 |
| materialize | 1.07x | 2.02x | 1.58x | 1.10x | 2.56x | 1.39x | 28/30 |
| perft_bulk_d4 | 1.17x | 3.85x | 1.48x | 1.27x | 2.01x | 1.88x | 30/30 |
| perft_full_d3 | 1.06x | 1.66x | 1.77x | 1.42x | 2.22x | 2.09x | 30/30 |
| fen_parse | 1.15x | 1.36x | 1.44x | 1.37x | 1.73x | 1.55x | 30/30 |
| fen_write_native_ep | 1.06x | 4.31x | 9.09x | 1.81x | 28.30x | 2.29x | 30/30 |
| snapshot | 1.09x | 1.47x | 1.12x | 2.35x | 1.16x | 0.95x | 21/30 |
| san_parse_all | 0.98x | 1.81x | N/A | 3.78x | N/A | N/A | 12/12 |
| san_render_native | 0.90x | 10.55x | N/A | 2.58x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.10x | 1.15x | 5.98x | 1.15x | 1.31x | 27/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.18x | N/A | N/A | 5/6 |
| startpos | 0.62x | 0.90x | 47.59x | 94.60x | 132.50x | 107.41x | 4/5 |
| chess960_materialize | 1.21x | 1.49x | 1.32x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.18x | 3.22x | 1.65x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.87x | 34.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 19.21x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.13x | 5.59x | 1.73x | 1.09x | 2.03x | 2.06x | 10/10 |
| corpus_materialize | 0.97x | 1.56x | 1.48x | 1.17x | 1.84x | 1.59x | 10/10 |
| corpus_uci_sorted | 1.05x | 1.92x | 2.61x | 1.84x | 2.65x | 1.27x | 10/10 |
| replay_trusted_native | 0.99x | 0.59x | 1.55x | 1.56x | 1.67x | 1.62x | 8/10 |
| replay_checked_native | 0.96x | 2.77x | 1.16x | 8.68x | 8.63x | 9.81x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
