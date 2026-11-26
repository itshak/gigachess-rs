| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.28x | 1.76x | 1.56x | 2.19x | 1.83x | 30/30 |
| materialize | 0.96x | 1.97x | 1.51x | 1.66x | 2.28x | 1.58x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.10x | 1.63x | 1.28x | 2.44x | 1.98x | 30/30 |
| perft_full_d3 | 1.02x | 1.61x | 1.92x | 1.76x | 2.62x | 1.97x | 30/30 |
| fen_parse | 0.97x | 1.17x | 1.29x | 1.04x | 1.48x | 1.24x | 29/30 |
| fen_write_native_ep | 1.00x | 5.43x | 10.85x | 1.94x | 31.94x | 2.61x | 30/30 |
| snapshot | 1.83x | 2.08x | 1.15x | 4.35x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 0.98x | 1.83x | N/A | 4.21x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.36x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.66x | 1.05x | 3.21x | 1.05x | 1.32x | 29/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 0.98x | 0.83x | 68.27x | 115.02x | 157.93x | 133.12x | 4/5 |
| chess960_materialize | 1.01x | 1.63x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.40x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 41.26x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 25.39x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 6.88x | 2.68x | 1.37x | 3.08x | 2.89x | 10/10 |
| corpus_materialize | 1.01x | 1.47x | 1.33x | 1.12x | 1.78x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.37x | 3.09x | 2.25x | 3.01x | 1.59x | 10/10 |
| replay_trusted_native | 0.94x | 0.66x | 1.21x | 1.21x | 1.67x | 1.88x | 8/10 |
| replay_checked_native | 0.98x | 2.15x | 1.05x | 8.40x | 8.21x | 9.37x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
