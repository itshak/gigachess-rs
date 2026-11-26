| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.85x | 1.72x | 1.78x | 2.44x | 2.03x | 30/30 |
| materialize | 1.04x | 2.10x | 1.62x | 1.06x | 2.24x | 1.42x | 27/30 |
| perft_bulk_d4 | 0.99x | 3.70x | 1.48x | 1.24x | 1.83x | 1.78x | 30/30 |
| perft_full_d3 | 1.13x | 1.41x | 1.68x | 1.23x | 1.78x | 1.87x | 30/30 |
| fen_parse | 1.10x | 1.30x | 1.50x | 1.40x | 2.02x | 1.56x | 30/30 |
| fen_write_native_ep | 1.05x | 4.11x | 14.00x | 1.71x | 28.58x | 2.23x | 30/30 |
| snapshot | 0.96x | 1.59x | 1.10x | 2.32x | 1.12x | 1.04x | 26/30 |
| san_parse_all | 1.01x | 1.75x | N/A | 3.64x | N/A | N/A | 12/12 |
| san_render_native | 1.13x | 11.57x | N/A | 2.64x | N/A | N/A | 12/12 |
| apply_children_full | 1.10x | 1.13x | 1.21x | 5.71x | 1.16x | 1.35x | 28/30 |
| make_unmake_full | 1.11x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 0.85x | 1.18x | 75.55x | 133.67x | 182.36x | 127.14x | 5/5 |
| chess960_materialize | 1.12x | 1.81x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.07x | 3.32x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.12x | 39.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.61x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 5.81x | 1.82x | 1.19x | 2.22x | 2.47x | 10/10 |
| corpus_materialize | 1.11x | 1.57x | 1.56x | 1.22x | 2.07x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.05x | 1.83x | 2.53x | 1.92x | 2.55x | 1.37x | 10/10 |
| replay_trusted_native | 1.25x | 0.71x | 1.82x | 1.74x | 1.88x | 1.80x | 8/10 |
| replay_checked_native | 1.24x | 2.62x | 1.15x | 9.60x | 8.66x | 10.23x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
