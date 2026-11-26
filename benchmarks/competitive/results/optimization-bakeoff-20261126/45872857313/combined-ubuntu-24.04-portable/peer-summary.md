| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.10x | 1.59x | 1.44x | 2.02x | 1.42x | 30/30 |
| materialize | 0.99x | 2.10x | 1.86x | 1.35x | 2.49x | 1.59x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.11x | 1.68x | 1.31x | 2.12x | 1.83x | 30/30 |
| perft_full_d3 | 1.00x | 1.38x | 1.60x | 1.36x | 1.89x | 1.75x | 30/30 |
| fen_parse | 1.18x | 1.67x | 1.58x | 1.29x | 2.39x | 1.36x | 30/30 |
| fen_write_native_ep | 1.03x | 2.87x | 10.03x | 1.67x | 28.07x | 2.01x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.22x | 3.04x | 1.02x | 0.97x | 24/30 |
| san_parse_all | 0.99x | 2.05x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.21x | N/A | 2.23x | N/A | N/A | 12/12 |
| apply_children_full | 0.88x | 1.98x | 1.38x | 4.02x | 1.27x | 1.33x | 30/30 |
| make_unmake_full | 0.88x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 66.23x | 112.45x | 176.80x | 109.31x | 5/5 |
| chess960_materialize | 0.99x | 1.65x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.29x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 28.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 3.82x | 1.93x | 1.28x | 2.10x | 2.55x | 10/10 |
| corpus_materialize | 1.01x | 1.62x | 1.64x | 1.23x | 2.15x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.20x | 3.27x | 2.47x | 3.16x | 1.80x | 10/10 |
| replay_trusted_native | 0.84x | 0.78x | 1.52x | 1.69x | 2.25x | 2.10x | 8/10 |
| replay_checked_native | 1.01x | 2.32x | 1.16x | 8.56x | 8.25x | 9.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
