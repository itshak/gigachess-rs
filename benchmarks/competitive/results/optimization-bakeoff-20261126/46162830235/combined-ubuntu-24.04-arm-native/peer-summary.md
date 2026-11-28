| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.48x | 1.69x | 1.82x | 2.20x | 2.19x | 30/30 |
| materialize | 1.02x | 1.99x | 1.56x | 1.36x | 2.19x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.02x | 1.63x | 1.49x | 2.10x | 2.15x | 30/30 |
| perft_full_d3 | 1.13x | 1.67x | 1.81x | 1.50x | 2.22x | 2.09x | 30/30 |
| fen_parse | 1.02x | 1.25x | 1.33x | 1.31x | 1.76x | 1.42x | 30/30 |
| fen_write_native_ep | 0.99x | 4.55x | 9.32x | 1.56x | 30.23x | 2.39x | 30/30 |
| snapshot | 0.98x | 1.05x | 1.09x | 1.45x | 1.08x | 0.96x | 24/30 |
| san_parse_all | 0.99x | 1.84x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.44x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.24x | 1.48x | 3.99x | 1.19x | 1.19x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.59x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.53x | 129.23x | 172.22x | 139.61x | 5/5 |
| chess960_materialize | 1.01x | 1.73x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.02x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.10x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.31x | 1.41x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 4.74x | 1.83x | 1.47x | 2.17x | 2.35x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.53x | 1.21x | 1.92x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.09x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 1.00x | 0.63x | 1.65x | 2.00x | 2.25x | 2.28x | 8/10 |
| replay_checked_native | 1.01x | 2.33x | 1.15x | 7.65x | 6.92x | 8.85x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
