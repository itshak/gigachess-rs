| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.46x | 1.86x | 1.61x | 2.20x | 1.94x | 30/30 |
| materialize | 1.01x | 2.06x | 1.37x | 1.60x | 2.43x | 1.65x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.55x | 1.17x | 2.14x | 1.83x | 29/30 |
| perft_full_d3 | 1.00x | 1.43x | 1.56x | 1.45x | 2.01x | 1.67x | 30/30 |
| fen_parse | 1.03x | 1.30x | 1.55x | 1.33x | 1.77x | 1.38x | 30/30 |
| fen_write_native_ep | 1.02x | 3.98x | 7.74x | 1.65x | 30.91x | 2.16x | 30/30 |
| snapshot | 1.13x | 2.07x | 1.13x | 4.93x | 1.12x | 1.12x | 30/30 |
| san_parse_all | 1.00x | 1.76x | N/A | 5.29x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 9.74x | N/A | 4.22x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.11x | 1.37x | 10.61x | 1.34x | 1.66x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.01x | 77.94x | 160.44x | 206.11x | 161.22x | 5/5 |
| chess960_materialize | 1.00x | 1.59x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 2.56x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 39.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 26.64x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.85x | 1.89x | 1.00x | 2.16x | 2.07x | 9/10 |
| corpus_materialize | 0.99x | 1.40x | 1.20x | 1.07x | 1.71x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.83x | 2.21x | 1.88x | 2.15x | 1.55x | 10/10 |
| replay_trusted_native | 0.98x | 0.85x | 1.53x | 1.62x | 2.16x | 2.46x | 9/10 |
| replay_checked_native | 1.16x | 2.35x | 1.27x | 9.42x | 9.43x | 10.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
