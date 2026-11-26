| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.31x | 1.64x | 1.77x | 2.13x | 2.12x | 30/30 |
| materialize | 1.00x | 1.96x | 1.54x | 1.36x | 2.16x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.03x | 3.96x | 1.61x | 1.48x | 2.07x | 2.11x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.60x | 1.33x | 1.97x | 1.86x | 30/30 |
| fen_parse | 1.00x | 1.24x | 1.33x | 1.27x | 1.75x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.60x | 9.41x | 1.63x | 30.04x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.05x | 1.06x | 1.45x | 1.06x | 0.96x | 21/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.43x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.00x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.56x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.20x | 129.58x | 173.44x | 139.47x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.97x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 25.21x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 18.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.86x | 1.84x | 1.46x | 2.15x | 2.38x | 10/10 |
| corpus_materialize | 1.01x | 1.62x | 1.54x | 1.24x | 1.90x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.11x | 2.90x | 2.09x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.63x | 1.66x | 2.01x | 2.26x | 2.30x | 8/10 |
| replay_checked_native | 1.01x | 2.35x | 1.14x | 7.73x | 6.90x | 9.02x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
