| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.92x | 1.67x | 1.42x | 2.19x | 1.73x | 30/30 |
| materialize | 1.04x | 2.10x | 1.61x | 1.64x | 2.48x | 1.92x | 30/30 |
| perft_bulk_d4 | 1.03x | 3.33x | 1.37x | 1.04x | 2.04x | 1.65x | 29/30 |
| perft_full_d3 | 1.00x | 1.45x | 1.70x | 1.62x | 2.11x | 1.83x | 30/30 |
| fen_parse | 1.06x | 1.38x | 1.32x | 1.15x | 1.45x | 1.12x | 30/30 |
| fen_write_native_ep | 0.99x | 4.59x | 9.88x | 1.89x | 27.50x | 2.51x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.33x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 0.99x | 2.11x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.32x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.00x | 1.28x | 3.84x | 1.26x | 2.12x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.13x | N/A | N/A | 5/6 |
| startpos | 0.89x | 1.12x | 90.67x | 165.33x | 195.44x | 155.07x | 5/5 |
| chess960_materialize | 1.00x | 1.65x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.35x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.98x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.85x | 16.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.25x | 1.62x | 0.89x | 1.95x | 1.81x | 8/10 |
| corpus_materialize | 1.01x | 1.35x | 1.26x | 1.04x | 1.65x | 1.13x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 2.97x | 2.22x | 2.86x | 1.55x | 10/10 |
| replay_trusted_native | 0.98x | 0.89x | 1.56x | 1.55x | 2.15x | 2.51x | 9/10 |
| replay_checked_native | 1.04x | 2.09x | 1.11x | 8.69x | 8.24x | 9.84x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
