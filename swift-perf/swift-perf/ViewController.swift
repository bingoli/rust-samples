//
//  ViewController.swift
//  swift-perf
//
//  Created by bingoli on 2020/2/14.
//  Copyright © 2020 bingoli.cn. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
        
        /*
        hello_devworld()

        let documentPaths = NSSearchPathForDirectoriesInDomains(FileManager.SearchPathDirectory.documentDirectory, FileManager.SearchPathDomainMask.userDomainMask, true)
        let dbPath = documentPaths[0] + "/demo.db"
        run_sqlite_perf_test(dbPath)
        */
        
        run_tokio_perf_test()
    }


}

