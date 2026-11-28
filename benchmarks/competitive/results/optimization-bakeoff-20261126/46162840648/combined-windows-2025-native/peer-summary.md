| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 5.20x | 1.81x | 1.74x | 2.49x | 2.25x | 30/30 |
| materialize | 0.98x | 2.06x | 1.33x | 1.22x | 2.28x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.30x | 1.67x | 1.43x | 2.43x | 2.28x | 30/30 |
| perft_full_d3 | 1.21x | 1.73x | 1.83x | 1.62x | 2.39x | 2.31x | 30/30 |
| fen_parse | 1.00x | 1.41x | 1.48x | 1.44x | 2.40x | 1.56x | 30/30 |
| fen_write_native_ep | 1.02x | 4.36x | 8.24x | 1.62x | 32.29x | 2.00x | 30/30 |
| snapshot | 0.99x | 1.87x | 1.05x | 4.07x | 1.00x | 1.01x | 21/30 |
| san_parse_all | 0.99x | 2.11x | N/A | 5.59x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.16x | N/A | 3.81x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.39x | 1.44x | 9.64x | 1.53x | 1.75x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.43x | 160.58x | 231.51x | 170.72x | 5/5 |
| chess960_materialize | 0.98x | 1.60x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.93x | 1.55x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 26.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.29x | 1.30x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 20.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.80x | 1.93x | 1.48x | 2.68x | 2.65x | 10/10 |
| corpus_materialize | 0.98x | 1.55x | 1.27x | 1.13x | 1.98x | 1.36x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.75x | 2.19x | 1.83x | 2.17x | 1.39x | 10/10 |
| replay_trusted_native | 0.97x | 0.82x | 1.56x | 2.01x | 2.51x | 2.94x | 9/10 |
| replay_checked_native | 0.92x | 2.31x | 1.04x | 7.88x | 8.11x | 9.13x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
