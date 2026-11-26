| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 2.87x | 1.49x | 1.33x | 1.88x | 1.29x | 29/30 |
| materialize | 1.01x | 2.14x | 1.66x | 1.27x | 2.36x | 1.58x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.02x | 1.53x | 1.18x | 1.92x | 1.73x | 30/30 |
| perft_full_d3 | 1.01x | 1.45x | 1.84x | 1.38x | 2.02x | 1.84x | 30/30 |
| fen_parse | 1.01x | 1.60x | 1.58x | 1.28x | 1.96x | 1.30x | 30/30 |
| fen_write_native_ep | 1.03x | 2.71x | 8.56x | 1.46x | 25.74x | 2.00x | 30/30 |
| snapshot | 1.01x | 1.67x | 1.20x | 2.83x | 1.16x | 1.02x | 28/30 |
| san_parse_all | 0.99x | 1.98x | N/A | 3.96x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.67x | N/A | 2.28x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.73x | 1.18x | 3.63x | 1.02x | 1.05x | 27/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 59.93x | 82.04x | 119.82x | 81.36x | 5/5 |
| chess960_materialize | 1.04x | 1.57x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.02x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 31.83x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.13x | 23.55x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 4.40x | 1.83x | 1.12x | 1.92x | 2.88x | 9/10 |
| corpus_materialize | 1.07x | 1.85x | 1.75x | 1.28x | 2.46x | 1.52x | 10/10 |
| corpus_uci_sorted | 1.06x | 2.13x | 2.85x | 2.29x | 2.93x | 1.62x | 10/10 |
| replay_trusted_native | 1.00x | 0.60x | 1.30x | 1.35x | 1.83x | 1.65x | 8/10 |
| replay_checked_native | 1.01x | 1.76x | 1.01x | 7.03x | 6.96x | 8.22x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
