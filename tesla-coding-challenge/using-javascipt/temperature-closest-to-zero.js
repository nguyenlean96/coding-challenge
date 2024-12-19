function computeClosestToZero(ts) {
  let minPos = Infinity, minNeg = -Infinity;

  for (let i = 0; i < ts.length; i++) {
    if (ts[i] < 0) {
      if (ts[i] < minNeg) {
        minNeg = ts[i]
      }
    } else {
      if (ts[i] < minPos) {
        minPos = ts[i]
      }
    }
  }

  console.log('minNeg', minNeg, 'minPos', minPos)
  if (Math.abs(minNeg) <= minPos) {
    return minNeg
  }
  return minPos
}

// Test cases
console.log('min', computeClosestToZero([3, -1, -2, 4, 5])); // -1
// console.log(computeClosestToZero([-2, 2])); // 2
// console.log(computeClosestToZero([-10, -20, 1, 5])); // 1
// console.log(computeClosestToZero([0])); // 0
// console.log(computeClosestToZero([-3, -5, -7])); // -3
// console.log(computeClosestToZero([5, 2, 8, 9])); // 2
// console.log(computeClosestToZero([0, -1, 1])); // 0
// console.log(computeClosestToZero([-5, 5, 0])); // 0
// console.log(computeClosestToZero([-10, 15, -2, 2])); // 2
// console.log(computeClosestToZero([10000, -9999])); // -9999

// Edge cases
// console.log(computeClosestToZero([])); // 0
// console.log(computeClosestToZero([10000])); // 10000
// console.log(computeClosestToZero([-10000])); // -10000
// console.log(computeClosestToZero([-9999, 9999])); // 9999
// console.log(computeClosestToZero([-3, -2, -1])); // -1
// console.log(computeClosestToZero([1, -1, -1, 2])); // 1
// console.log(computeClosestToZero([-500, -1000, 500])); // 500
// console.log(computeClosestToZero([-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5])); // 0
// console.log(computeClosestToZero([6, 5, 4, 3, 2, 1, -1, -2, -3, -4, -5])); // 1
// console.log(computeClosestToZero([-2, -2, -2])); // -2
