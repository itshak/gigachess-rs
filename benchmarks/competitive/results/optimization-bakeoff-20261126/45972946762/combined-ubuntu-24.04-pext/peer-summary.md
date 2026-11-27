| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.48x | 1.88x | 1.64x | 2.52x | 1.92x | 30/30 |
| materialize | 1.01x | 2.27x | 1.75x | 1.79x | 2.76x | 1.94x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.80x | 1.52x | 1.19x | 2.35x | 1.85x | 29/30 |
| perft_full_d3 | 1.00x | 1.34x | 1.61x | 1.51x | 1.98x | 2.13x | 30/30 |
| fen_parse | 1.00x | 1.27x | 1.36x | 1.17x | 1.46x | 1.13x | 29/30 |
| fen_write_native_ep | 0.99x | 4.59x | 10.21x | 1.86x | 27.46x | 2.12x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 0.92x | 1.71x | N/A | 3.87x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.05x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.17x | 1.38x | 4.09x | 1.36x | 1.71x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.12x | 88.65x | 162.98x | 190.87x | 156.40x | 5/5 |
| chess960_materialize | 1.13x | 1.80x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.54x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 36.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 23.03x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.44x | 2.03x | 1.12x | 2.48x | 2.28x | 10/10 |
| corpus_materialize | 1.04x | 1.42x | 1.30x | 1.08x | 1.72x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.26x | 2.93x | 2.22x | 2.85x | 1.54x | 10/10 |
| replay_trusted_native | 0.94x | 0.83x | 1.56x | 1.52x | 2.07x | 2.41x | 9/10 |
| replay_checked_native | 0.98x | 2.11x | 1.17x | 9.06x | 8.64x | 10.19x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
