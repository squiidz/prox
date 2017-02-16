import { Component, OnInit } from '@angular/core';

import { User } from './user';
import { UsersService } from './users.service';

@Component({
  selector: 'app-users',
  templateUrl: './users.component.html',
  styleUrls: ['./users.component.css']
})
export class UsersComponent implements OnInit {
  users: User[];
  newUser: User = new User("", 0);

  constructor(private usersService: UsersService) { }

  ngOnInit() { this.loadUsers() }

  loadUsers() {
    this.usersService.getRemoteUsers()
      .subscribe(
        users => this.users = users,
        err => { console.log(err) });
    console.log(this.users);
  }

  pushUser(name: string, age: number) {
    let user = new User(name, age);
    this.usersService.addRemoteUser(user)
      .subscribe(
        user => this.users.push(user),
        err => { console.log(err) });
    this.newUser = new User("", 0);
  }

  removeUser(id: number) {
    this.usersService.deleteUser(id)
      .subscribe(
        user => console.log(user),
        err => { console.log(err) });
    //this.loadUsers();
  }
}
