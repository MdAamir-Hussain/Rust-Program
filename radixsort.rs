fn radix_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    
    let mut max = arr[0];
    for &num in arr.iter() {
        if num > max {
            max = num;
        }
    }
    
    let mut exp = 1;
    let mut output = vec![0; arr.len()];
    
    while max / exp > 0 {
        let mut count = vec![0; 10];
        
        for &num in arr.iter() {
            count[(num / exp % 10) as usize] += 1;
        }
        
        for i in 1..10 {
            count[i] += count[i - 1];
        }
        
        for &num in arr.iter().rev() {
            let index = (num / exp % 10) as usize;
            output[count[index] - 1] = num;
            count[index] -= 1;
        }
        
        for (i, &num) in output.iter().enumerate() {
            arr[i] = num;
        }
        
        exp *= 10;
    }
}

fn main() {
    let mut arr = [170, 45, 75, 90, 802, 24, 2, 66];
    println!("Original array: {:?}", arr);
    radix_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
