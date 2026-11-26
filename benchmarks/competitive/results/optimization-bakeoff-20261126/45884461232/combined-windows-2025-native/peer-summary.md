| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.40x | 1.83x | 1.59x | 2.17x | 1.94x | 30/30 |
| materialize | 1.04x | 2.08x | 1.42x | 1.63x | 2.52x | 1.66x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.93x | 1.53x | 1.13x | 2.06x | 1.77x | 28/30 |
| perft_full_d3 | 1.01x | 2.13x | 1.54x | 1.44x | 2.02x | 1.66x | 30/30 |
| fen_parse | 0.96x | 1.22x | 1.39x | 1.30x | 1.70x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.28x | 7.64x | 1.69x | 29.94x | 2.16x | 30/30 |
| snapshot | 1.13x | 2.16x | 1.18x | 4.95x | 1.20x | 1.14x | 30/30 |
| san_parse_all | 1.00x | 1.76x | N/A | 5.31x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.65x | N/A | 4.09x | N/A | N/A | 12/12 |
| apply_children_full | 0.80x | 1.69x | 1.10x | 8.50x | 1.08x | 1.34x | 27/30 |
| make_unmake_full | 0.88x | N/A | N/A | 1.06x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.00x | 77.90x | 161.04x | 206.80x | 168.48x | 4/5 |
| chess960_materialize | 0.98x | 1.56x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.57x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 40.94x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.28x | 33.62x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.77x | 1.89x | 0.96x | 2.14x | 2.05x | 8/10 |
| corpus_materialize | 1.03x | 1.41x | 1.21x | 1.08x | 1.75x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.81x | 2.22x | 1.87x | 2.20x | 1.54x | 10/10 |
| replay_trusted_native | 0.87x | 0.70x | 1.28x | 1.38x | 1.84x | 2.11x | 8/10 |
| replay_checked_native | 0.92x | 2.09x | 1.09x | 8.56x | 8.32x | 9.66x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
