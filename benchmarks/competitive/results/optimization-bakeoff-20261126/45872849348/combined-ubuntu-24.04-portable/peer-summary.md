| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.21x | 1.62x | 1.48x | 2.09x | 1.47x | 30/30 |
| materialize | 1.00x | 2.15x | 1.88x | 1.35x | 2.52x | 1.70x | 30/30 |
| perft_bulk_d4 | 0.94x | 2.96x | 1.58x | 1.23x | 1.99x | 1.72x | 30/30 |
| perft_full_d3 | 0.99x | 1.35x | 1.55x | 1.24x | 1.85x | 1.67x | 30/30 |
| fen_parse | 1.12x | 1.65x | 1.55x | 1.21x | 2.20x | 1.29x | 30/30 |
| fen_write_native_ep | 1.04x | 2.83x | 9.42x | 1.69x | 27.20x | 2.02x | 30/30 |
| snapshot | 1.00x | 1.70x | 1.23x | 3.07x | 1.04x | 1.07x | 30/30 |
| san_parse_all | 0.99x | 2.07x | N/A | 4.36x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.61x | N/A | 2.22x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 2.01x | 1.41x | 4.16x | 1.30x | 1.34x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.32x | 67.01x | 109.59x | 181.72x | 116.86x | 5/5 |
| chess960_materialize | 0.99x | 1.68x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.24x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 28.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.11x | 21.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.86x | 1.93x | 1.28x | 2.11x | 2.57x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.65x | 1.18x | 2.09x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.22x | 3.23x | 2.44x | 3.13x | 1.75x | 10/10 |
| replay_trusted_native | 0.99x | 0.91x | 1.69x | 1.98x | 2.61x | 2.44x | 8/10 |
| replay_checked_native | 0.99x | 2.29x | 1.13x | 7.83x | 8.15x | 9.01x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
