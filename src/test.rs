mod tests {
    #[test]
    fn test_reading() {
        use crate::read_file;
        let filename = String::from("test_input.csv");
        let original_data = read_file(&filename);
        assert_eq!(
            original_data.0, 3,
            "Duplicate Nodes Are Not Being Recognized"
        );
        assert_eq!(
            (original_data.1)[0],
            (0, 1, 100),
            "File Not Being Read Properly"
        );
        assert_eq!(
            (original_data.1).len(),
            4,
            "Edges Where Start and End Notes Are The Same Are Being Considered"
        );
    }
    #[test]
    fn test_duplicate_handling() {
        use crate::read_file;
        use crate::unique_edges;
        let filename = String::from("test_input.csv");
        let original_data = read_file(&filename);
        let unique_data = unique_edges(original_data.1);
        assert_eq!(
            unique_data.len(),
            3,
            "Duplicate Edges Are Not Being Averaged"
        );
        assert_eq!(
            unique_data[1],
            (1, 2, 250),
            "Averages Are Not Being Properly Calculated"
        );
    }
    #[test]
    fn test_seconds_conversion() {
        use crate::seconds_to_minutes_and_seconds;
        assert_eq!(
            seconds_to_minutes_and_seconds(179),
            (2, 59),
            "Conversion Done Wrongly"
        );
    }
    #[test]
    fn test_shortest_paths_and_minutes_conversion() {
        use crate::graph::Edge;
        use crate::graph::Vertex;
        use crate::read_file;
        use crate::seconds_to_minutes_and_seconds;
        use crate::unique_edges;
        use crate::Graph;

        let filename = String::from("test_input.csv");
        let start_location = String::from("Location One"); //insert starting point here. A Txt file is generated the first time around ("commented away in this code") showing what each vertex number corresponds to//
        let end_location = String::from("Location Three");
        let original_data = read_file(&filename);
        let unique_data = unique_edges(original_data.1);
        let n = original_data.0;
        let edges: Vec<Edge> = unique_data;
        let graph = Graph::create_directed(n, &edges);
        let start: Vertex = *(original_data.2)
            .get(&start_location)
            .expect("Invalid Start Location");
        let end: usize = *(original_data.2)
            .get(&end_location)
            .expect("Invalid End Location");
        assert_eq!(
            seconds_to_minutes_and_seconds(graph.find_shortest_path(start, end)),
            (5, 50),
            "Shortest Paths Algorithm Not Counting Distances If Prior Edges Did Not Exist"
        );
    }
    #[test]
    #[should_panic]
    fn test_wrong_input() {
        use crate::graph::Edge;
        use crate::graph::Vertex;
        use crate::read_file;
        use crate::seconds_to_minutes_and_seconds;
        use crate::unique_edges;
        use crate::Graph;

        let filename = String::from("test_input.csv");
        let start_location = String::from("Location Oneeeeee"); //insert starting point here. A Txt file is generated the first time around ("commented away in this code") showing what each vertex number corresponds to//
        let end_location = String::from("Location Three");
        let original_data = read_file(&filename);
        let unique_data = unique_edges(original_data.1);
        let n = original_data.0;
        let edges: Vec<Edge> = unique_data;
        let graph = Graph::create_directed(n, &edges);
        let start: Vertex = *(original_data.2)
            .get(&start_location)
            .expect("Invalid Start Location");
        let end: usize = *(original_data.2)
            .get(&end_location)
            .expect("Invalid End Location");
    }
    #[test]
    #[should_panic]
    fn test_directed_graph() {
        use crate::graph::Edge;
        use crate::graph::Vertex;
        use crate::read_file;
        use crate::seconds_to_minutes_and_seconds;
        use crate::unique_edges;
        use crate::Graph;

        let filename = String::from("test_input.csv");
        let start_location = String::from("Location Three"); //insert starting point here. A Txt file is generated the first time around ("commented away in this code") showing what each vertex number corresponds to//
        let end_location = String::from("Location One");
        let original_data = read_file(&filename);
        let unique_data = unique_edges(original_data.1);
        let n = original_data.0;
        let edges: Vec<Edge> = unique_data;
        let graph = Graph::create_directed(n, &edges);
        let start: Vertex = *(original_data.2)
            .get(&start_location)
            .expect("Invalid Start Location");
        let end: usize = *(original_data.2)
            .get(&end_location)
            .expect("Invalid End Location");
        assert_eq!(
            seconds_to_minutes_and_seconds(graph.find_shortest_path(start, end)),
            (5, 5),
            "Shortest Paths Algorithm Not Counting Distances If Prior Edges Did Not Exist"
        );
    }
}
