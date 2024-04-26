fn heap_sort(arr: &mut [i32]) {
    // Build a max heap
    build_max_heap(arr);

    // Extract elements from the heap one by one
    for i in (1..arr.len()).rev() {
        arr.swap(0, i); // Move the root element to the end
        max_heapify(&mut arr[..i], 0); // Adjust heap after swapping
    }
}

fn build_max_heap(arr: &mut [i32]) {
    let n = arr.len();
    for i in (0..=(n / 2)).rev() {
        max_heapify(arr, i);
    }
}

fn max_heapify(arr: &mut [i32], mut i: usize) {
    let mut largest = i;
    let n = arr.len();
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    // Check if left child is larger than root
    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    // Check if right child is larger than largest so far
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    // If largest is not root
    if largest != i {
        arr.swap(i, largest);

        // Recursively heapify the affected sub-tree
        max_heapify(arr, largest);
    }
}

fn main() {
    let mut arr = [12, 11, 13, 5, 6, 7];
    println!("Original array: {:?}", arr);
    heap_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
